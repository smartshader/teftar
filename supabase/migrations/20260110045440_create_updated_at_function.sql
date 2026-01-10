-- Create function to automatically update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create clients table
CREATE TABLE clients (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,

    -- Client type
    client_type VARCHAR(10) NOT NULL CHECK (client_type IN ('company', 'person')),

    -- Company/Person details
    company_name VARCHAR(255),
    person_name VARCHAR(255),
    email VARCHAR(255),

    -- Phone numbers (stored as JSONB array)
    -- Structure: [{ type: 'business' | 'mobile' | 'fax', number: '...' }]
    phone_numbers JSONB DEFAULT '[]'::jsonb,

    -- Address
    country VARCHAR(100),
    address_line1 VARCHAR(255),
    address_line2 VARCHAR(255),
    city VARCHAR(100),
    province VARCHAR(100),
    postal_code VARCHAR(20),

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Constraints
    CONSTRAINT client_name_check CHECK (
        (client_type = 'company' AND company_name IS NOT NULL) OR
        (client_type = 'person' AND person_name IS NOT NULL)
    )
);

-- Create index on user_id for faster lookups
CREATE INDEX idx_clients_user_id ON clients(user_id);

-- Create index on email for search
CREATE INDEX idx_clients_email ON clients(email);

-- Create updated_at trigger
CREATE TRIGGER update_clients_updated_at
    BEFORE UPDATE ON clients
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Enable Row Level Security
ALTER TABLE clients ENABLE ROW LEVEL SECURITY;

-- RLS Policies: Users can only access their own clients
CREATE POLICY "Users can view their own clients"
    ON clients FOR SELECT
    USING (auth.uid() = user_id);

CREATE POLICY "Users can create their own clients"
    ON clients FOR INSERT
    WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can update their own clients"
    ON clients FOR UPDATE
    USING (auth.uid() = user_id)
    WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can delete their own clients"
    ON clients FOR DELETE
    USING (auth.uid() = user_id);
