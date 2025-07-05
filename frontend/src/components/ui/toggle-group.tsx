import * as React from "react"
import { CompactToggle, CompactToggleProps } from "./compact-toggle"
import { cn } from "@/lib/utils"

export interface ToggleItem {
  key: string
  label: string
  description?: string
  value: boolean
  disabled?: boolean
  loading?: boolean
  variant?: CompactToggleProps['variant']
}

export interface ToggleGroupProps {
  title?: string
  description?: string
  items: ToggleItem[]
  onToggle: (key: string, value: boolean) => void
  className?: string
  size?: CompactToggleProps['size']
  orientation?: 'vertical' | 'horizontal'
}

export function ToggleGroup({
  title,
  description,
  items,
  onToggle,
  className,
  size = "md",
  orientation = "vertical"
}: ToggleGroupProps) {
  return (
    <div className={cn("space-y-4", className)}>
      {(title || description) && (
        <div className="space-y-1">
          {title && (
            <h3 className="text-lg font-medium leading-none">{title}</h3>
          )}
          {description && (
            <p className="text-sm text-muted-foreground">{description}</p>
          )}
        </div>
      )}
      
      <div className={cn(
        "space-y-4",
        orientation === "horizontal" && "flex flex-wrap gap-4 space-y-0"
      )}>
        {items.map((item) => (
          <div
            key={item.key}
            className={cn(
              "flex items-center justify-between py-2",
              orientation === "horizontal" && "py-0"
            )}
          >
            <CompactToggle
              checked={item.value}
              onCheckedChange={(checked) => onToggle(item.key, checked)}
              loading={item.loading}
              disabled={item.disabled}
              label={item.label}
              description={item.description}
              size={size}
              variant={item.variant}
            />
          </div>
        ))}
      </div>
    </div>
  )
}