import { useEffect } from "react";
import { useNavigate } from "react-router";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import type { Route } from "./+types/clients";
import { config } from "~/lib/config";
import { useAuthStore } from "~/lib/stores/auth";
import type { Client } from "~/lib/types/client";
import { DashboardLayout } from "~/components/layouts/dashboard-layout";
import { ClientFormDialog } from "~/components/clients/client-form-dialog";
import { Button } from "~/components/ui/button";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "~/components/ui/table";
import { Trash2 } from "lucide-react";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Clients - Teftar" },
    { name: "description", content: "Manage your clients" },
  ];
}

export default function Clients() {
  const navigate = useNavigate();
  const accessToken = useAuthStore((state) => state.accessToken);
  const queryClient = useQueryClient();

  useEffect(() => {
    const token =
      accessToken ||
      (typeof window !== "undefined"
        ? localStorage.getItem("access_token")
        : null);
    if (!token) {
      navigate("/signin");
    }
  }, [accessToken, navigate]);

  const { data: clients = [], isLoading } = useQuery({
    queryKey: ["clients"],
    queryFn: async () => {
      const token =
        accessToken ||
        (typeof window !== "undefined"
          ? localStorage.getItem("access_token")
          : null);
      const response = await fetch(`${config.apiUrl}/clients`, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      if (!response.ok) {
        throw new Error("Failed to fetch clients");
      }

      return response.json() as Promise<Client[]>;
    },
    enabled:
      !!accessToken ||
      (typeof window !== "undefined" && !!localStorage.getItem("access_token")),
  });

  const deleteClientMutation = useMutation({
    mutationFn: async (clientId: string) => {
      const token =
        accessToken ||
        (typeof window !== "undefined"
          ? localStorage.getItem("access_token")
          : null);
      const response = await fetch(`${config.apiUrl}/clients/${clientId}`, {
        method: "DELETE",
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      if (!response.ok) {
        const error = await response.json();
        throw new Error(error.error || "Failed to delete client");
      }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["clients"] });
    },
  });

  const handleDelete = (clientId: string, clientName: string) => {
    if (window.confirm(`Are you sure you want to delete ${clientName}?`)) {
      deleteClientMutation.mutate(clientId);
    }
  };

  const getClientName = (client: Client) => {
    return client.client_type === "company"
      ? client.company_name
      : client.person_name;
  };

  const getClientPhones = (client: Client) => {
    if (!client.phone_numbers || client.phone_numbers.length === 0) {
      return "-";
    }
    return client.phone_numbers.map((p) => p.number).join(", ");
  };

  return (
    <DashboardLayout>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-3xl font-bold">Clients</h1>
            <p className="text-muted-foreground">
              Manage your clients and their information
            </p>
          </div>
          <ClientFormDialog />
        </div>

        {isLoading ? (
          <div className="text-center py-12 text-muted-foreground">
            Loading clients...
          </div>
        ) : clients.length === 0 ? (
          <div className="text-center py-12">
            <p className="text-muted-foreground mb-4">
              No clients yet. Add your first client to get started.
            </p>
            <ClientFormDialog
              trigger={<Button>Add Your First Client</Button>}
            />
          </div>
        ) : (
          <div className="border rounded-lg">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Name</TableHead>
                  <TableHead>Type</TableHead>
                  <TableHead>Email</TableHead>
                  <TableHead>Phone</TableHead>
                  <TableHead>City</TableHead>
                  <TableHead className="text-right">Actions</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {clients.map((client) => (
                  <TableRow key={client.id}>
                    <TableCell className="font-medium">
                      {getClientName(client)}
                    </TableCell>
                    <TableCell className="capitalize">
                      {client.client_type}
                    </TableCell>
                    <TableCell>{client.email || "-"}</TableCell>
                    <TableCell>{getClientPhones(client)}</TableCell>
                    <TableCell>{client.city || "-"}</TableCell>
                    <TableCell className="text-right">
                      <Button
                        variant="ghost"
                        size="icon"
                        onClick={() =>
                          handleDelete(
                            client.id,
                            getClientName(client) || "this client",
                          )
                        }
                        disabled={deleteClientMutation.isPending}
                      >
                        <Trash2 className="h-4 w-4 text-destructive" />
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </div>
        )}

        {deleteClientMutation.isError && (
          <div className="text-sm text-destructive">
            {deleteClientMutation.error.message}
          </div>
        )}
      </div>
    </DashboardLayout>
  );
}
