import { useState } from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { config } from "~/lib/config";
import { useAuthStore } from "~/lib/stores/auth";
import type { CreateClientRequest, PhoneNumber } from "~/lib/types/client";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "~/components/ui/dialog";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { Label } from "~/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "~/components/ui/select";
import { Plus, X } from "lucide-react";

interface CompanyFormDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function CompanyFormDialog({
  open,
  onOpenChange,
}: CompanyFormDialogProps) {
  const [formData, setFormData] = useState<
    Omit<CreateClientRequest, "client_type">
  >({
    company_name: "",
    email: "",
    phone_numbers: [],
    country: "",
    address_line1: "",
    city: "",
    province: "",
    postal_code: "",
  });

  const [phoneNumbers, setPhoneNumbers] = useState<PhoneNumber[]>([]);
  const accessToken = useAuthStore((state) => state.accessToken);
  const queryClient = useQueryClient();

  const createClientMutation = useMutation({
    mutationFn: async (data: CreateClientRequest) => {
      const token =
        accessToken ||
        (typeof window !== "undefined"
          ? localStorage.getItem("access_token")
          : null);
      const response = await fetch(`${config.apiUrl}/clients`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify(data),
      });

      if (!response.ok) {
        const text = await response.text();
        try {
          const error = JSON.parse(text);
          throw new Error(error.error || "Failed to create client");
        } catch {
          throw new Error(`Failed to create client: ${text}`);
        }
      }

      return response.json();
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["clients"] });
      onOpenChange(false);
      resetForm();
    },
  });

  const resetForm = () => {
    setFormData({
      company_name: "",
      email: "",
      phone_numbers: [],
      country: "",
      address_line1: "",
      city: "",
      province: "",
      postal_code: "",
    });
    setPhoneNumbers([]);
  };

  const addPhoneNumber = () => {
    setPhoneNumbers([...phoneNumbers, { type: "business", number: "" }]);
  };

  const removePhoneNumber = (index: number) => {
    setPhoneNumbers(phoneNumbers.filter((_, i) => i !== index));
  };

  const updatePhoneNumber = (
    index: number,
    field: keyof PhoneNumber,
    value: string,
  ) => {
    const updated = [...phoneNumbers];
    updated[index] = { ...updated[index], [field]: value };
    setPhoneNumbers(updated);
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const submitData: CreateClientRequest = {
      client_type: "company",
      ...formData,
      phone_numbers: phoneNumbers.filter((p) => p.number.trim() !== ""),
    };
    createClientMutation.mutate(submitData);
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>Add Company Client</DialogTitle>
          <DialogDescription>
            Create a new company client. Fill in the information below.
          </DialogDescription>
        </DialogHeader>
        <form onSubmit={handleSubmit} className="space-y-6">
          <div className="space-y-2">
            <Label htmlFor="company-name">Company Name *</Label>
            <Input
              id="company-name"
              value={formData.company_name || ""}
              onChange={(e) =>
                setFormData({ ...formData, company_name: e.target.value })
              }
              required
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="email">Email</Label>
            <Input
              id="email"
              type="email"
              value={formData.email || ""}
              onChange={(e) =>
                setFormData({ ...formData, email: e.target.value })
              }
            />
          </div>

          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <Label>Phone Numbers</Label>
              <Button
                type="button"
                variant="outline"
                size="sm"
                onClick={addPhoneNumber}
              >
                <Plus className="h-4 w-4 mr-1" />
                Add Phone
              </Button>
            </div>
            {phoneNumbers.map((phone, index) => (
              <div key={index} className="flex gap-2">
                <Select
                  value={phone.type}
                  onValueChange={(value) =>
                    updatePhoneNumber(index, "type", value)
                  }
                >
                  <SelectTrigger className="w-[140px]">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="business">Business</SelectItem>
                    <SelectItem value="mobile">Mobile</SelectItem>
                    <SelectItem value="fax">Fax</SelectItem>
                  </SelectContent>
                </Select>
                <Input
                  value={phone.number}
                  onChange={(e) =>
                    updatePhoneNumber(index, "number", e.target.value)
                  }
                  placeholder="Phone number"
                />
                <Button
                  type="button"
                  variant="ghost"
                  size="icon"
                  onClick={() => removePhoneNumber(index)}
                >
                  <X className="h-4 w-4" />
                </Button>
              </div>
            ))}
          </div>

          <div className="space-y-4">
            <h3 className="font-medium">Address</h3>
            <div className="grid grid-cols-2 gap-4">
              <div className="space-y-2 col-span-2">
                <Label htmlFor="country">Country</Label>
                <Input
                  id="country"
                  value={formData.country || ""}
                  onChange={(e) =>
                    setFormData({ ...formData, country: e.target.value })
                  }
                />
              </div>
              <div className="space-y-2 col-span-2">
                <Label htmlFor="address1">Address Line 1</Label>
                <Input
                  id="address1"
                  value={formData.address_line1 || ""}
                  onChange={(e) =>
                    setFormData({ ...formData, address_line1: e.target.value })
                  }
                />
              </div>
              <div className="space-y-2 col-span-2">
                <Label htmlFor="address2">Address Line 2</Label>
                <Input
                  id="address2"
                  value={formData.address_line2 || ""}
                  onChange={(e) =>
                    setFormData({ ...formData, address_line2: e.target.value })
                  }
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="city">City</Label>
                <Input
                  id="city"
                  value={formData.city || ""}
                  onChange={(e) =>
                    setFormData({ ...formData, city: e.target.value })
                  }
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="province">Province/State</Label>
                <Input
                  id="province"
                  value={formData.province || ""}
                  onChange={(e) =>
                    setFormData({ ...formData, province: e.target.value })
                  }
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="postal">Postal Code</Label>
                <Input
                  id="postal"
                  value={formData.postal_code || ""}
                  onChange={(e) =>
                    setFormData({ ...formData, postal_code: e.target.value })
                  }
                />
              </div>
            </div>
          </div>

          <div className="flex justify-end gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
            >
              Cancel
            </Button>
            <Button type="submit" disabled={createClientMutation.isPending}>
              {createClientMutation.isPending ? "Creating..." : "Create Client"}
            </Button>
          </div>

          {createClientMutation.isError && (
            <div className="text-sm text-destructive">
              {createClientMutation.error.message}
            </div>
          )}
        </form>
      </DialogContent>
    </Dialog>
  );
}
