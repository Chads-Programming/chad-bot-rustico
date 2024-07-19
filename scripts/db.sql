CREATE EXTENSION IF NOT EXISTS "pgcrypto";


CREATE TABLE MEMBER (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE MEMBER
ALTER COLUMN discord_id SET NOT NULL;


CREATE TABLE WALLET (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    member_id UUID UNIQUE NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (member_id) REFERENCES member(id) ON DELETE CASCADE
);

-- handle updated_at column auto update function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


-- trigger for updated_at for member
CREATE TRIGGER update_member_updated_at
BEFORE UPDATE ON member
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();


-- trigger for updated_at for wallet
CREATE TRIGGER update_wallet_updated_at
BEFORE UPDATE ON wallet
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();


CREATE OR REPLACE FUNCTION create_member_with_wallet(member_name VARCHAR, member_discord_id VARCHAR)
RETURNS UUID AS $$
DECLARE
    new_member_id UUID;
BEGIN
    BEGIN
        -- Insert the new member and return his ID
        INSERT INTO member (name, discord_id)
        VALUES (member_name, member_discord_id)
        RETURNING id INTO new_member_id;

        -- Insert the wallet with the amount in 1000 for the new member
        INSERT INTO wallet (member_id, amount)
        VALUES (new_member_id, 100);

        -- Commit the transaction returning the new member ID
        RETURN new_member_id;
    EXCEPTION
        WHEN OTHERS THEN
            -- Revert the transacion if error exist
            RAISE;
    END;
END;
$$ LANGUAGE plpgsql;
