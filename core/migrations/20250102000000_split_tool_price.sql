-- 拆分 tools 表的 price 列为 price_amount 和 price_currency
-- 以支持多货币

-- 1. 添加新列
ALTER TABLE tools ADD COLUMN IF NOT EXISTS price_amount BIGINT;
ALTER TABLE tools ADD COLUMN IF NOT EXISTS price_currency VARCHAR(3) DEFAULT 'CNY';

-- 2. 迁移现有数据（假设现有 price 是 CNY，转换为分）
UPDATE tools 
SET price_amount = (price * 100)::BIGINT,
    price_currency = 'CNY'
WHERE price_amount IS NULL;

-- 3. 设置新列为 NOT NULL
ALTER TABLE tools ALTER COLUMN price_amount SET NOT NULL;
ALTER TABLE tools ALTER COLUMN price_currency SET NOT NULL;

-- 4. 添加约束
ALTER TABLE tools ADD CONSTRAINT chk_price_amount_non_negative 
    CHECK (price_amount >= 0);

ALTER TABLE tools ADD CONSTRAINT chk_price_currency_valid 
    CHECK (price_currency IN ('CNY', 'USD'));

-- 5. 删除旧的 price 列
ALTER TABLE tools DROP COLUMN IF EXISTS price;

-- 6. 删除旧的约束（如果存在）
ALTER TABLE tools DROP CONSTRAINT IF EXISTS chk_price_non_negative;

-- 7. 创建索引（可选）
CREATE INDEX IF NOT EXISTS idx_tools_price_currency ON tools(price_currency);
