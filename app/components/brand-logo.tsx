import { FileText } from "lucide-react";

interface BrandLogoProps {
  size?: "sm" | "md" | "lg";
  linkTo?: string;
  className?: string;
}

export function BrandLogo({ size = "md", linkTo, className = "" }: BrandLogoProps) {
  const sizes = {
    sm: { icon: "h-5 w-5", text: "text-lg" },
    md: { icon: "h-6 w-6", text: "text-xl" },
    lg: { icon: "h-8 w-8", text: "text-2xl" },
  };

  const content = (
    <>
      <FileText className={sizes[size].icon} />
      <span className={`${sizes[size].text} font-semibold`}>Teftar</span>
    </>
  );

  const baseClasses = "flex items-center space-x-2";
  const linkClasses = linkTo ? "hover:opacity-80 transition-opacity" : "";
  const finalClasses = `${baseClasses} ${linkClasses} ${className}`.trim();

  if (linkTo) {
    return (
      <a href={linkTo} className={finalClasses}>
        {content}
      </a>
    );
  }

  return <div className={finalClasses}>{content}</div>;
}
