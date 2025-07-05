import * as React from "react"
import * as SwitchPrimitive from "@radix-ui/react-switch"
import { Loader2, Check, X } from "lucide-react"
import { cn } from "@/lib/utils"

export interface InlineToggleProps
  extends React.ComponentPropsWithoutRef<typeof SwitchPrimitive.Root> {
  loading?: boolean
  showIcon?: boolean
  onLabel?: string
  offLabel?: string
  variant?: "default" | "minimal"
}

const InlineToggle = React.forwardRef<
  React.ElementRef<typeof SwitchPrimitive.Root>,
  InlineToggleProps
>(({ 
  className, 
  loading = false, 
  disabled, 
  showIcon = false,
  onLabel = "On",
  offLabel = "Off",
  variant = "default",
  checked,
  ...props 
}, ref) => {
  return (
    <div className="flex items-center space-x-2">
      {variant === "default" && (
        <span className={cn(
          "text-xs font-medium transition-colors",
          checked ? "text-muted-foreground" : "text-foreground"
        )}>
          {offLabel}
        </span>
      )}
      
      <SwitchPrimitive.Root
        className={cn(
          "peer inline-flex h-5 w-9 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=unchecked]:bg-input",
          variant === "minimal" && "h-4 w-7",
          className
        )}
        disabled={disabled || loading}
        checked={checked}
        {...props}
        ref={ref}
      >
        <SwitchPrimitive.Thumb
          className={cn(
            "pointer-events-none block rounded-full bg-background shadow-lg ring-0 transition-transform data-[state=unchecked]:translate-x-0 flex items-center justify-center",
            variant === "minimal" 
              ? "h-3 w-3 data-[state=checked]:translate-x-3" 
              : "h-4 w-4 data-[state=checked]:translate-x-4"
          )}
        >
          {loading ? (
            <Loader2 className="h-2 w-2 animate-spin text-muted-foreground" />
          ) : showIcon ? (
            checked ? (
              <Check className="h-2 w-2 text-primary" />
            ) : (
              <X className="h-2 w-2 text-muted-foreground" />
            )
          ) : null}
        </SwitchPrimitive.Thumb>
      </SwitchPrimitive.Root>
      
      {variant === "default" && (
        <span className={cn(
          "text-xs font-medium transition-colors",
          checked ? "text-foreground" : "text-muted-foreground"
        )}>
          {onLabel}
        </span>
      )}
    </div>
  )
})
InlineToggle.displayName = "InlineToggle"

export { InlineToggle }