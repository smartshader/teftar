import { useState } from "react";
import { Button } from "~/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "~/components/ui/dropdown-menu";
import { Plus, Building2, User } from "lucide-react";
import { CompanyFormDialog } from "./company-form-dialog";
import { PersonFormDialog } from "./person-form-dialog";

export function AddClientMenu() {
  const [companyDialogOpen, setCompanyDialogOpen] = useState(false);
  const [personDialogOpen, setPersonDialogOpen] = useState(false);

  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button>
            <Plus className="h-4 w-4 mr-2" />
            Add Client
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuItem onClick={() => setCompanyDialogOpen(true)}>
            <Building2 className="h-4 w-4 mr-2" />
            Company
          </DropdownMenuItem>
          <DropdownMenuItem onClick={() => setPersonDialogOpen(true)}>
            <User className="h-4 w-4 mr-2" />
            Person
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      <CompanyFormDialog
        open={companyDialogOpen}
        onOpenChange={setCompanyDialogOpen}
      />
      <PersonFormDialog
        open={personDialogOpen}
        onOpenChange={setPersonDialogOpen}
      />
    </>
  );
}
