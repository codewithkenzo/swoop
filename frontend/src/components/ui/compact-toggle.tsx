import * as React from "react"
import * as SwitchPrimitive from "@radix-ui/react-switch"
import { Loader2 } from "lucide-react"
import { cn } from "@/lib/utils"

export interface CompactToggleProps
  extends React.ComponentPropsWithoutRef<typeof SwitchPrimitive.Root> {
  loading?: boolean
  label?: string
  description?: string
  size?: "sm" | "md"
  variant?: "default" | "success" | "warning"
}

const CompactToggle = React.forwardRef<
  React.ElementRef<typeof SwitchPrimitive.Root>,
  CompactToggleProps
>(({ 
  className, 
  loading = false, 
  disabled, 
  label,
  description,
  size = "md",
  variant = "default",
  ...props 
}, ref) => {
  const sizeClasses = {
    sm: "h-4 w-7",
    md: "h-6 w-11"
  }
  
  const thumbSizeClasses = {
    sm: "h-3 w-3 data-[state=checked]:translate-x-3",
    md: "h-5 w-5 data-[state=checked]:translate-x-5"
  }

  const variantClasses = {
    default: "data-[state=checked]:bg-primary",
    success: "data-[state=checked]:bg-green-500",
    warning: "data-[state=checked]:bg-yellow-500"
  }

  const toggle = (
    <SwitchPrimitive.Root
      className={cn(
        "peer inline-flex shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 data-[state=unchecked]:bg-input",
        sizeClasses[size],
        variantClasses[variant],
        className
      )}
      disabled={disabled || loading}
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
            size === "sm" ? "h-2 w-2" : "h-3 w-3"
          )} />
        )}
      </SwitchPrimitive.Thumb>
    </SwitchPrimitive.Root>
  )

  if (!label) {
    return toggle
  }

  return (
    <div className="flex items-center space-x-3">
      {toggle}
      <div className="space-y-1">
        <label className={cn(
          "font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
          size === "sm" ? "text-xs" : "text-sm"
        )}>
          {label}
        </label>
        {description && (
          <p className={cn(
            "text-muted-foreground",
            size === "sm" ? "text-xs" : "text-xs"
          )}>
            {description}
          </p>
        )}
      </div>
    </div>
  )
})
CompactToggle.displayName = "CompactToggle"

export { CompactToggle }