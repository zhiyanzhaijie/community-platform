//! 服务列表查询用例

use domain::{
    profession::ProfessionType,
    service::{Service, ServiceRepository},
};
use shared::Result;
use tracing::{info, instrument};

/// 服务列表查询输入
#[derive(Debug)]
pub struct ListServicesInput {
    pub profession_type: Option<ProfessionType>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// 服务列表查询输出
#[derive(Debug)]
pub struct ListServicesOutput {
    pub services: Vec<Service>,
    pub total_count: u64,
}

/// 服务列表查询用例
#[instrument(
    name = "list_services",
    skip(service_repo),
    fields(
        profession_type = ?input.profession_type,
        limit = ?input.limit,
        offset = ?input.offset
    )
)]
pub async fn list_services(
    service_repo: &impl ServiceRepository,
    input: ListServicesInput,
) -> Result<ListServicesOutput> {
    info!("开始查询服务列表");

    let limit = input.limit.unwrap_or(20); // 默认20条
    let offset = input.offset.unwrap_or(0);

    let services = match input.profession_type {
        Some(profession_type) => {
            service_repo
                .find_available_by_profession(profession_type, Some(limit), Some(offset))
                .await?
        }
        None => {
            service_repo
                .find_available_services(Some(limit), Some(offset))
                .await?
        }
    };

    let total_count = service_repo.count_available_services().await?;

    info!(
        services_count = services.len(),
        total_count = total_count,
        "服务列表查询完成"
    );

    Ok(ListServicesOutput {
        services,
        total_count,
    })
}

/// 搜索服务用例
#[instrument(
    name = "search_services",
    skip(service_repo),
    fields(
        keyword = %keyword,
        profession_type = ?profession_type
    )
)]
pub async fn search_services(
    service_repo: &impl ServiceRepository,
    keyword: String,
    profession_type: Option<ProfessionType>,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Vec<Service>> {
    info!("开始搜索服务");

    let limit = limit.unwrap_or(20);
    let offset = offset.unwrap_or(0);

    let services = service_repo
        .search_services(&keyword, profession_type, Some(limit), Some(offset))
        .await?;

    info!(
        keyword = %keyword,
        results_count = services.len(),
        "服务搜索完成"
    );

    Ok(services)
}