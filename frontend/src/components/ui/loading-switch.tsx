import * as React from "react"
import * as SwitchPrimitive from "@radix-ui/react-switch"
import { Loader2 } from "lucide-react"
import { cn } from "@/lib/utils"

export interface LoadingSwitchProps
  extends React.ComponentPropsWithoutRef<typeof SwitchPrimitive.Root> {
  loading?: boolean
}

const LoadingSwitch = React.forwardRef<
  React.ElementRef<typeof SwitchPrimitive.Root>,
  LoadingSwitchProps
>(({ className, loading = false, disabled, ...props }, ref) => (
  <SwitchPrimitive.Root
    className={cn(
      "peer inline-flex h-6 w-11 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=unchecked]:bg-input",
      className
    )}
    disabled={disabled || loading}
    {...props}
    ref={ref}
  >
    <SwitchPrimitive.Thumb
      className={cn(
        "pointer-events-none block h-5 w-5 rounded-full bg-background shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0 flex items-center justify-center"
      )}
    >
      {loading && (
        <Loader2 className="h-3 w-3 animate-spin text-muted-foreground" />
      )}
    </SwitchPrimitive.Thumb>
  </SwitchPrimitive.Root>
))
LoadingSwitch.displayName = "LoadingSwitch"

export { LoadingSwitch }