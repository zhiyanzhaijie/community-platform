use std::sync::Arc;

use api::v1::member::AppState;
use infra::{
    create_pool, init_tracing, Argon2PasswordHasher, PasswordHasher, PostgresMemberRepository,
};
use shared::AppConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 加载配置
    let config = AppConfig::load()?;

    // 初始化日志
    init_tracing(&config.log.level, &config.log.format);
    tracing::info!("Starting server...");

    // 创建数据库连接池
    let pool = create_pool(
        &config.database.url,
        config.database.max_connections,
        config.database.min_connections,
        config.database.connect_timeout,
        config.database.acquire_timeout,
    )
    .await?;
    tracing::info!("Database pool created");

    // 初始化应用状态
    let member_repo: Arc<dyn domain::member::MemberRepository> =
        Arc::new(PostgresMemberRepository::new(pool.clone()));
    let password_hasher: Arc<dyn PasswordHasher> = Arc::new(Argon2PasswordHasher::new());
    let state = AppState {
        member_repo,
        password_hasher,
        config: Arc::new(config.clone()),
    };

    // 构建路由
    let app = api::routes::app_routes(state);

    // 启动服务器
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
