-- 初始化数据库架构
-- 创建时间: 2024-01-01
-- 描述: 创建社区交易平台的核心表结构

-- ============================================
-- 1. 会员表 (members)
-- ============================================
CREATE TABLE IF NOT EXISTS members (
    id UUID PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(50) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    role VARCHAR(20) NOT NULL DEFAULT 'regular',
    managed_professions JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT chk_status CHECK (status IN ('active', 'inactive', 'banned')),
    CONSTRAINT chk_role CHECK (role IN ('regular', 'decider', 'admin'))
);

-- 创建索引
CREATE INDEX idx_members_email ON members(email);
CREATE INDEX idx_members_username ON members(username);
CREATE INDEX idx_members_status ON members(status);
CREATE INDEX idx_members_role ON members(role);

-- ============================================
-- 2. ISU账户表 (isu_accounts)
-- ============================================
CREATE TABLE IF NOT EXISTS isu_accounts (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL REFERENCES members(id) ON DELETE CASCADE,
    balance DECIMAL(20, 2) NOT NULL DEFAULT 0.00,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT chk_balance_non_negative CHECK (balance >= 0),
    CONSTRAINT uq_isu_accounts_owner UNIQUE (owner_id)
);

-- 创建索引
CREATE INDEX idx_isu_accounts_owner_id ON isu_accounts(owner_id);

-- ============================================
-- 3. ISU交易记录表 (isu_transactions)
-- ============================================
CREATE TABLE IF NOT EXISTS isu_transactions (
    id UUID PRIMARY KEY,
    from_account_id UUID NOT NULL REFERENCES isu_accounts(id),
    to_account_id UUID NOT NULL REFERENCES isu_accounts(id),
    amount DECIMAL(20, 2) NOT NULL,
    transaction_type VARCHAR(50) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT chk_amount_positive CHECK (amount > 0),
    CONSTRAINT chk_different_accounts CHECK (from_account_id != to_account_id),
    CONSTRAINT chk_transaction_type CHECK (
        transaction_type IN ('service_payment', 'tool_rental', 'initial_balance', 'admin_adjustment')
    )
);

-- 创建索引
CREATE INDEX idx_isu_transactions_from_account ON isu_transactions(from_account_id);
CREATE INDEX idx_isu_transactions_to_account ON isu_transactions(to_account_id);
CREATE INDEX idx_isu_transactions_created_at ON isu_transactions(created_at DESC);
CREATE INDEX idx_isu_transactions_type ON isu_transactions(transaction_type);

-- ============================================
-- 4. 职业标准表 (profession_standards)
-- ============================================
CREATE TABLE IF NOT EXISTS profession_standards (
    id UUID PRIMARY KEY,
    profession_type VARCHAR(50) NOT NULL,
    isu_rate DECIMAL(10, 2) NOT NULL,
    description TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_by UUID NOT NULL REFERENCES members(id),
    updated_by UUID NOT NULL REFERENCES members(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT chk_isu_rate_positive CHECK (isu_rate > 0),
    CONSTRAINT uq_profession_standards_type UNIQUE (profession_type)
);

-- 创建索引
CREATE INDEX idx_profession_standards_type ON profession_standards(profession_type);
CREATE INDEX idx_profession_standards_active ON profession_standards(is_active);

-- ============================================
-- 5. 职业标准变更历史表 (profession_standard_history)
-- ============================================
CREATE TABLE IF NOT EXISTS profession_standard_history (
    id UUID PRIMARY KEY,
    standard_id UUID NOT NULL REFERENCES profession_standards(id) ON DELETE CASCADE,
    action VARCHAR(50) NOT NULL,
    old_rate DECIMAL(10, 2),
    new_rate DECIMAL(10, 2),
    reason TEXT NOT NULL,
    changed_by UUID NOT NULL REFERENCES members(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT chk_history_action CHECK (
        action IN ('created', 'rate_updated', 'activated', 'deactivated')
    )
);

-- 创建索引
CREATE INDEX idx_profession_history_standard_id ON profession_standard_history(standard_id);
CREATE INDEX idx_profession_history_created_at ON profession_standard_history(created_at DESC);

-- ============================================
-- 6. 服务表 (services)
-- ============================================
CREATE TABLE IF NOT EXISTS services (
    id UUID PRIMARY KEY,
    provider_id UUID NOT NULL REFERENCES members(id) ON DELETE CASCADE,
    profession_type VARCHAR(50) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    estimated_hours DECIMAL(10, 2) NOT NULL,
    total_isu DECIMAL(20, 2) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'available',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT chk_estimated_hours_positive CHECK (estimated_hours > 0),
    CONSTRAINT chk_total_isu_positive CHECK (total_isu > 0),
    CONSTRAINT chk_service_status CHECK (
        status IN ('available', 'in_progress', 'completed', 'cancelled')
    )
);

-- 创建索引
CREATE INDEX idx_services_provider_id ON services(provider_id);
CREATE INDEX idx_services_profession_type ON services(profession_type);
CREATE INDEX idx_services_status ON services(status);
CREATE INDEX idx_services_created_at ON services(created_at DESC);

-- ============================================
-- 7. 工具表 (tools)
-- ============================================
CREATE TABLE IF NOT EXISTS tools (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL REFERENCES members(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    category VARCHAR(100) NOT NULL,
    price DECIMAL(20, 2) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'available',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT chk_price_non_negative CHECK (price >= 0),
    CONSTRAINT chk_tool_status CHECK (
        status IN ('available', 'rented', 'unavailable')
    )
);

-- 创建索引
CREATE INDEX idx_tools_owner_id ON tools(owner_id);
CREATE INDEX idx_tools_category ON tools(category);
CREATE INDEX idx_tools_status ON tools(status);
CREATE INDEX idx_tools_created_at ON tools(created_at DESC);

-- ============================================
-- 8. 交易表 (transactions)
-- ============================================
CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY,
    buyer_id UUID NOT NULL REFERENCES members(id),
    seller_id UUID NOT NULL REFERENCES members(id),
    item_type VARCHAR(20) NOT NULL,
    item_id UUID NOT NULL,
    isu_amount DECIMAL(20, 2) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    
    CONSTRAINT chk_isu_amount_positive CHECK (isu_amount > 0),
    CONSTRAINT chk_different_parties CHECK (buyer_id != seller_id),
    CONSTRAINT chk_item_type CHECK (item_type IN ('service', 'tool')),
    CONSTRAINT chk_transaction_status CHECK (
        status IN ('pending', 'confirmed', 'in_progress', 'completed', 'cancelled', 'disputed')
    )
);

-- 创建索引
CREATE INDEX idx_transactions_buyer_id ON transactions(buyer_id);
CREATE INDEX idx_transactions_seller_id ON transactions(seller_id);
CREATE INDEX idx_transactions_item ON transactions(item_type, item_id);
CREATE INDEX idx_transactions_status ON transactions(status);
CREATE INDEX idx_transactions_created_at ON transactions(created_at DESC);

-- ============================================
-- 9. 创建更新时间触发器函数
-- ============================================
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- 为所有表添加更新时间触发器
CREATE TRIGGER update_members_updated_at BEFORE UPDATE ON members
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_isu_accounts_updated_at BEFORE UPDATE ON isu_accounts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_profession_standards_updated_at BEFORE UPDATE ON profession_standards
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_services_updated_at BEFORE UPDATE ON services
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_tools_updated_at BEFORE UPDATE ON tools
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_transactions_updated_at BEFORE UPDATE ON transactions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================
-- 10. 插入初始数据（可选）
-- ============================================

-- 插入默认管理员账户（密码: admin123，需要实际运行时通过bcrypt加密）
-- 注意：实际部署时应该通过应用程序API或管理脚本创建管理员
-- INSERT INTO members (id, email, username, password_hash, status, role)
-- VALUES (
--     gen_random_uuid(),
--     'admin@example.com',
--     'admin',
--     '$2b$12$...', -- 替换为实际的bcrypt哈希
--     'active',
--     'admin'
-- );

-- 初始化默认职业标准
INSERT INTO profession_standards (id, profession_type, isu_rate, description, created_by, updated_by)
SELECT 
    gen_random_uuid(),
    profession_type,
    default_rate,
    description,
    (SELECT id FROM members WHERE role = 'admin' LIMIT 1), -- 需要先有管理员
    (SELECT id FROM members WHERE role = 'admin' LIMIT 1)
FROM (VALUES
    ('plumber', 50.00, '管道工默认费率: 50 ISU/小时'),
    ('electrician', 60.00, '电工默认费率: 60 ISU/小时'),
    ('carpenter', 55.00, '木工默认费率: 55 ISU/小时'),
    ('painter', 45.00, '油漆工默认费率: 45 ISU/小时'),
    ('cleaner', 30.00, '清洁工默认费率: 30 ISU/小时'),
    ('gardener', 40.00, '园丁默认费率: 40 ISU/小时'),
    ('mechanic', 65.00, '机械师默认费率: 65 ISU/小时'),
    ('teacher', 70.00, '教师默认费率: 70 ISU/小时'),
    ('programmer', 100.00, '程序员默认费率: 100 ISU/小时'),
    ('designer', 80.00, '设计师默认费率: 80 ISU/小时')
) AS t(profession_type, default_rate, description)
WHERE EXISTS (SELECT 1 FROM members WHERE role = 'admin' LIMIT 1); -- 仅在有管理员时插入

COMMENT ON TABLE members IS '会员表 - 存储平台用户信息';
COMMENT ON TABLE isu_accounts IS 'ISU账户表 - 存储会员的ISU余额';
COMMENT ON TABLE isu_transactions IS 'ISU交易记录表 - 记录所有ISU流转';
COMMENT ON TABLE profession_standards IS '职业标准表 - 定义各职业的ISU费率';
COMMENT ON TABLE profession_standard_history IS '职业标准变更历史表 - 审计职业费率变更';
COMMENT ON TABLE services IS '服务表 - 会员提供的服务信息';
COMMENT ON TABLE tools IS '工具表 - 可租用的工具信息';
COMMENT ON TABLE transactions IS '交易表 - 记录服务和工具的交易';
