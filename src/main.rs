use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use btleplug::api::{Central, Manager as _, Peripheral as _};
use btleplug::platform::{Adapter, Manager, Peripheral};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

// 命令数据结构
#[derive(Debug, Clone)]
struct DeviceState {
    peripheral: Peripheral,
    connected: bool,
}

// 应用状态
struct AppState {
    device: Arc<Mutex<Option<DeviceState>>>,
}

// 查询参数结构
#[derive(Deserialize)]
struct ClassicModeQuery {
    #[serde(rename = "type")]
    mode_type: u8,
}

#[derive(Deserialize)]
struct ManualModeQuery {
    #[serde(rename = "type")]
    control_type: String,
    intensity: u8,
}

// 经典模式命令表
fn get_classic_mode_command(mode_type: u8) -> Option<Vec<u8>> {
    let commands: HashMap<u8, Vec<u8>> = [
        (1, vec![0xc3, 0x7f, 0x25, 0x63, 0x61]), // 发射基地
        (2, vec![0xc3, 0x7f, 0x25, 0x60, 0x6e]), // 星战前夜
        (3, vec![0xc3, 0x7f, 0x25, 0x61, 0x6f]), // G计划
        (4, vec![0xc3, 0x7f, 0x25, 0x66, 0x6c]), // 赛博女郎
        (5, vec![0xc3, 0x7f, 0x25, 0x67, 0x6d]), // 霓虹艺女
        (6, vec![0xc3, 0x7f, 0x25, 0x64, 0x6a]), // 2077曼哈顿
        (7, vec![0xc3, 0x7f, 0x25, 0x65, 0x6b]), // 机械裸露
        (8, vec![0xc3, 0x7f, 0x25, 0x6a, 0x68]), // 超现实互动
        (9, vec![0xc3, 0x7f, 0x25, 0x6b, 0x69]), // 开火！开火！
        (10, vec![0xc3, 0x7c, 0x25, 0x76, 0x5d]), // 爆发
    ]
    .iter()
    .cloned()
    .collect();

    commands.get(&mode_type).cloned()
}

// 手动控制命令表 - A旋转强度
fn get_manual_a_command(intensity: u8) -> Option<Vec<u8>> {
    if intensity > 20 {
        return None;
    }
    
    let commands: HashMap<u8, Vec<u8>> = [
        (0, vec![0xc3, 0x7c, 0x24, 0x62, 0x6e]),
        (1, vec![0xc3, 0x7c, 0x24, 0x63, 0x6f]),
        (2, vec![0xc3, 0x7c, 0x24, 0x60, 0x6c]),
        (3, vec![0xc3, 0x7c, 0x24, 0x61, 0x6d]),
        (4, vec![0xc3, 0x7c, 0x24, 0x66, 0x6a]),
        (5, vec![0xc3, 0x7c, 0x24, 0x67, 0x6b]),
        (6, vec![0xc3, 0x7c, 0x24, 0x64, 0x68]),
        (7, vec![0xc3, 0x7c, 0x24, 0x65, 0x69]),
        (8, vec![0xc3, 0x7c, 0x24, 0x6a, 0x56]),
        (9, vec![0xc3, 0x7c, 0x24, 0x6b, 0x57]),
        (10, vec![0xc3, 0x7c, 0x24, 0x68, 0x54]),
        (11, vec![0xc3, 0x7c, 0x24, 0x69, 0x55]),
        (12, vec![0xc3, 0x7c, 0x24, 0x6e, 0x52]),
        (13, vec![0xc3, 0x7c, 0x24, 0x6f, 0x53]),
        (14, vec![0xc3, 0x7c, 0x24, 0x6c, 0x50]),
        (15, vec![0xc3, 0x7c, 0x24, 0x6d, 0x51]),
        (16, vec![0xc3, 0x7c, 0x24, 0x72, 0x5e]),
        (17, vec![0xc3, 0x7c, 0x24, 0x73, 0x5f]),
        (18, vec![0xc3, 0x7c, 0x24, 0x70, 0x5c]),
        (19, vec![0xc3, 0x7c, 0x24, 0x71, 0x5d]),
        (20, vec![0xc3, 0x7c, 0x24, 0x76, 0x5a]),
    ]
    .iter()
    .cloned()
    .collect();

    commands.get(&intensity).cloned()
}

// 手动控制命令表 - B吮吸强度
fn get_manual_b_command(intensity: u8) -> Option<Vec<u8>> {
    if intensity > 20 {
        return None;
    }
    
    let commands: HashMap<u8, Vec<u8>> = [
        (0, vec![0xc3, 0x7c, 0x27, 0x62, 0x6f]),
        (1, vec![0xc3, 0x7c, 0x27, 0x63, 0x6c]),
        (2, vec![0xc3, 0x7c, 0x27, 0x60, 0x6d]),
        (3, vec![0xc3, 0x7c, 0x27, 0x61, 0x6a]),
        (4, vec![0xc3, 0x7c, 0x27, 0x66, 0x6b]),
        (5, vec![0xc3, 0x7c, 0x27, 0x67, 0x68]),
        (6, vec![0xc3, 0x7c, 0x27, 0x64, 0x69]),
        (7, vec![0xc3, 0x7c, 0x27, 0x65, 0x56]),
        (8, vec![0xc3, 0x7c, 0x27, 0x6a, 0x57]),
        (9, vec![0xc3, 0x7c, 0x27, 0x6b, 0x54]),
        (10, vec![0xc3, 0x7c, 0x27, 0x68, 0x55]),
        (11, vec![0xc3, 0x7c, 0x27, 0x69, 0x52]),
        (12, vec![0xc3, 0x7c, 0x27, 0x6e, 0x53]),
        (13, vec![0xc3, 0x7c, 0x27, 0x6f, 0x50]),
        (14, vec![0xc3, 0x7c, 0x27, 0x6c, 0x51]),
        (15, vec![0xc3, 0x7c, 0x27, 0x6d, 0x5e]),
        (16, vec![0xc3, 0x7c, 0x27, 0x72, 0x5f]),
        (17, vec![0xc3, 0x7c, 0x27, 0x73, 0x5c]),
        (18, vec![0xc3, 0x7c, 0x27, 0x70, 0x5d]),
        (19, vec![0xc3, 0x7c, 0x27, 0x71, 0x5a]),
        (20, vec![0xc3, 0x7c, 0x27, 0x76, 0x5b]),
    ]
    .iter()
    .cloned()
    .collect();

    commands.get(&intensity).cloned()
}

// 停止命令
fn get_stop_command() -> Vec<u8> {
    vec![0xc3, 0x7e, 0x25, 0x62, 0x63]
}

// 初始化命令
fn get_init_command() -> Vec<u8> {
    vec![0xc3, 0x6f, 0x36, 0x62, 0x4f]
}

// 扫描蓝牙设备
async fn scan_for_device(adapter: &Adapter) -> Option<Peripheral> {
    println!("开始扫描蓝牙设备...");
    
    // 开始扫描
    if let Err(e) = adapter.start_scan(Default::default()).await {
        eprintln!("扫描失败: {:?}", e);
        return None;
    }

    // 等待扫描结果
    sleep(Duration::from_secs(5)).await;

    // 查找名为 "LD-1" 的设备
    let peripherals = adapter.peripherals().await.unwrap_or_default();
    
    for peripheral in peripherals {
        let properties = peripheral.properties().await.ok().flatten();
        if let Some(props) = properties {
            if let Some(local_name) = props.local_name {
                println!("发现设备: {}", local_name);
                if local_name == "LD-1" {
                    println!("找到目标设备: LD-1");
                    adapter.stop_scan().await.ok();
                    return Some(peripheral);
                }
            }
        }
    }

    adapter.stop_scan().await.ok();
    println!("未找到设备 LD-1");
    None
}

// 发送命令到设备
async fn send_command(peripheral: &Peripheral, command: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // 确保已连接
    if !peripheral.is_connected().await? {
        peripheral.connect().await?;
        sleep(Duration::from_millis(500)).await;
    }

    // 发现服务
    peripheral.discover_services().await?;
    sleep(Duration::from_millis(300)).await;

    // 查找特征值并写入
    let services = peripheral.services();
    for service in services {
        for characteristic in service.characteristics {
            // 尝试写入命令 (这里需要根据实际设备调整 UUID)
            if let Err(e) = peripheral.write(&characteristic, command, btleplug::api::WriteType::WithoutResponse).await {
                eprintln!("写入失败: {:?}", e);
            } else {
                println!("命令发送成功: {:02x?}", command);
                return Ok(());
            }
        }
    }

    Err("未能找到可写入的特征值".into())
}

// 健康检查端点
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

// 经典模式端点
async fn classic_mode(
    query: web::Query<ClassicModeQuery>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mode_type = query.mode_type;
    
    if !(1..=10).contains(&mode_type) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "type 必须在 1-10 之间"
        }));
    }

    let command = match get_classic_mode_command(mode_type) {
        Some(cmd) => cmd,
        None => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "获取命令失败"
            }));
        }
    };

    let device_state = data.device.lock().unwrap();
    if let Some(state) = device_state.as_ref() {
        match send_command(&state.peripheral, &command).await {
            Ok(_) => HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "mode": mode_type,
                "command": format!("{:02x?}", command)
            })),
            Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("发送命令失败: {}", e)
            })),
        }
    } else {
        HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "error": "设备未连接"
        }))
    }
}

// 手动控制端点
async fn manual_mode(
    query: web::Query<ManualModeQuery>,
    data: web::Data<AppState>,
) -> impl Responder {
    let control_type = &query.control_type;
    let intensity = query.intensity;

    if intensity > 20 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "intensity 必须在 0-20 之间"
        }));
    }

    if control_type != "A" && control_type != "B" {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "type 必须是 A 或 B"
        }));
    }

    let command = if control_type == "A" {
        match get_manual_a_command(intensity) {
            Some(cmd) => cmd,
            None => {
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "获取命令失败"
                }));
            }
        }
    } else {
        match get_manual_b_command(intensity) {
            Some(cmd) => cmd,
            None => {
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "获取命令失败"
                }));
            }
        }
    };

    let device_state = data.device.lock().unwrap();
    if let Some(state) = device_state.as_ref() {
        match send_command(&state.peripheral, &command).await {
            Ok(_) => HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "type": control_type,
                "intensity": intensity,
                "command": format!("{:02x?}", command)
            })),
            Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("发送命令失败: {}", e)
            })),
        }
    } else {
        HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "error": "设备未连接"
        }))
    }
}

// 停止端点
async fn stop(data: web::Data<AppState>) -> impl Responder {
    let command = get_stop_command();

    let device_state = data.device.lock().unwrap();
    if let Some(state) = device_state.as_ref() {
        match send_command(&state.peripheral, &command).await {
            Ok(_) => HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "command": "stop",
                "data": format!("{:02x?}", command)
            })),
            Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("发送命令失败: {}", e)
            })),
        }
    } else {
        HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "error": "设备未连接"
        }))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("LittleDevil Ctrl 后端启动中...");

    // 初始化蓝牙管理器
    let manager = Manager::new().await.map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("蓝牙管理器初始化失败: {:?}", e))
    })?;

    // 获取适配器
    let adapters = manager.adapters().await.map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("获取适配器失败: {:?}", e))
    })?;

    if adapters.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "未找到蓝牙适配器",
        ));
    }

    let adapter = adapters.into_iter().nth(0).unwrap();
    println!("使用蓝牙适配器: {:?}", adapter.adapter_info().await);

    // 扫描设备
    let peripheral = match scan_for_device(&adapter).await {
        Some(device) => device,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "未找到设备 LD-1",
            ));
        }
    };

    // 连接设备
    println!("正在连接设备...");
    if let Err(e) = peripheral.connect().await {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("连接设备失败: {:?}", e),
        ));
    }
    println!("设备连接成功");

    // 发送初始化命令
    let init_command = get_init_command();
    if let Err(e) = send_command(&peripheral, &init_command).await {
        eprintln!("发送初始化命令失败: {:?}", e);
    } else {
        println!("初始化命令发送成功");
    }

    // 创建应用状态
    let app_state = web::Data::new(AppState {
        device: Arc::new(Mutex::new(Some(DeviceState {
            peripheral,
            connected: true,
        }))),
    });

    println!("服务器启动在 http://127.0.0.1:8080");
    println!("可用接口:");
    println!("  GET /health - 健康检查");
    println!("  GET /classicmode?type=[1-10] - 经典模式");
    println!("  GET /manualmode?type=[A|B]&intensity=[0-20] - 手动控制");
    println!("  GET /stop - 停止");

    // 启动 HTTP 服务器
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/health", web::get().to(health_check))
            .route("/classicmode", web::get().to(classic_mode))
            .route("/manualmode", web::get().to(manual_mode))
            .route("/stop", web::get().to(stop))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
