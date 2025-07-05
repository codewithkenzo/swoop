import * as React from "react"
import * as SwitchPrimitive from "@radix-ui/react-switch"
import { Loader2 } from "lucide-react"
import { cn } from "@/lib/utils"
import { Button } from "./button"

export interface QuickToggleProps
  extends React.ComponentPropsWithoutRef<typeof SwitchPrimitive.Root> {
  loading?: boolean
  icon?: React.ReactNode
  activeIcon?: React.ReactNode
  tooltip?: string
  variant?: "switch" | "button"
  size?: "sm" | "md" | "lg"
}

const QuickToggle = React.forwardRef<
  React.ElementRef<typeof SwitchPrimitive.Root>,
  QuickToggleProps
>(({ 
  className, 
  loading = false, 
  disabled, 
  icon,
  activeIcon,
  tooltip,
  variant = "switch",
  size = "sm",
  checked,
  onCheckedChange,
  ...props 
}, ref) => {
  if (variant === "button") {
    return (
      <Button
        variant={checked ? "default" : "outline"}
        size="sm"
        disabled={disabled || loading}
        onClick={() => onCheckedChange?.(!checked)}
        className={cn("h-8 w-8 p-0", className)}
        title={tooltip}
      >
        {loading ? (
          <Loader2 className="h-4 w-4 animate-spin" />
        ) : checked && activeIcon ? (
          activeIcon
        ) : (
          icon
        )}
      </Button>
    )
  }

  const sizeClasses = {
    sm: "h-4 w-7",
    md: "h-5 w-9", 
    lg: "h-6 w-11"
  }
  
  const thumbSizeClasses = {
    sm: "h-3 w-3 data-[state=checked]:translate-x-3",
    md: "h-4 w-4 data-[state=checked]:translate-x-4",
    lg: "h-5 w-5 data-[state=checked]:translate-x-5"
  }

  return (
    <div className="flex items-center space-x-2">
      {icon && (
        <div className={cn(
          "flex items-center justify-center",
          checked ? "text-primary" : "text-muted-foreground"
        )}>
          {icon}
        </div>
      )}
      
      <SwitchPrimitive.Root
        className={cn(
          "peer inline-flex shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=unchecked]:bg-input",
          sizeClasses[size],
          className
        )}
        disabled={disabled || loading}
        checked={checked}
        onCheckedChange={onCheckedChange}
        title={tooltip}
        {...props}
        ref={ref}
      >
        <SwitchPrimitive.Thumb
          className={cn(
            "pointer-events-none block rounded-full bg-background shadow-lg ring-0 transition-transform data-[state=unchecked]:translate-x-0 flex items-center justify-center",
            thumbSizeClasses[size]
          )}
        >
          {loading && (
            <Loader2 className={cn(
              "animate-spin text-muted-foreground",
              size === "sm" ? "h-2 w-2" : size === "md" ? "h-2.5 w-2.5" : "h-3 w-3"
            )} />
          )}
        </SwitchPrimitive.Thumb>
      </SwitchPrimitive.Root>
    </div>
  )
})
QuickToggle.displayName = "QuickToggle"

export { QuickToggle }