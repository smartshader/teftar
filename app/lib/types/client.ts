export type ClientType = "company" | "person";

export type PhoneNumberType = "business" | "mobile" | "fax";

export interface PhoneNumber {
  type: PhoneNumberType;
  number: string;
}

export interface Client {
  id: string;
  user_id: string;
  client_type: ClientType;
  company_name: string | null;
  person_name: string | null;
  email: string | null;
  phone_numbers: PhoneNumber[];
  country: string | null;
  address_line1: string | null;
  address_line2: string | null;
  city: string | null;
  province: string | null;
  postal_code: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateClientRequest {
  client_type: ClientType;
  company_name?: string;
  person_name?: string;
  email?: string;
  phone_numbers?: PhoneNumber[];
  country?: string;
  address_line1?: string;
  address_line2?: string;
  city?: string;
  province?: string;
  postal_code?: string;
}

export interface UpdateClientRequest extends Partial<CreateClientRequest> {}
