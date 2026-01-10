import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "~/components/ui/card";

interface AuthCardProps {
  title: string;
  description: string;
  children: React.ReactNode;
  variant?: "default" | "info";
}

export function AuthCard({
  title,
  description,
  children,
  variant = "default"
}: AuthCardProps) {
  const cardClassName = variant === "info"
    ? "border-blue-200 bg-blue-50/50 dark:bg-blue-950/20 dark:border-blue-900"
    : "";

  return (
    <Card className={cardClassName}>
      <CardHeader>
        <CardTitle>{title}</CardTitle>
        <CardDescription>{description}</CardDescription>
      </CardHeader>
      <CardContent>{children}</CardContent>
    </Card>
  );
}
