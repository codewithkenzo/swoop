import * as React from "react"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "./card"
import { ToggleGroup, ToggleItem } from "./toggle-group"
import { CompactToggle } from "./compact-toggle"
import { cn } from "@/lib/utils"

export interface SettingsCardProps {
  title: string
  description?: string
  children?: React.ReactNode
  className?: string
}

export interface ToggleSettingsCardProps extends SettingsCardProps {
  items: ToggleItem[]
  onToggle: (key: string, value: boolean) => void
  orientation?: 'vertical' | 'horizontal'
}

export interface SingleToggleCardProps extends SettingsCardProps {
  checked: boolean
  onCheckedChange: (checked: boolean) => void
  loading?: boolean
  disabled?: boolean
  toggleLabel?: string
  toggleDescription?: string
}

// Base settings card
export function SettingsCard({ 
  title, 
  description, 
  children, 
  className 
}: SettingsCardProps) {
  return (
    <Card className={cn(className)}>
      <CardHeader>
        <CardTitle>{title}</CardTitle>
        {description && <CardDescription>{description}</CardDescription>}
      </CardHeader>
      <CardContent>
        {children}
      </CardContent>
    </Card>
  )
}

// Settings card with multiple toggles
export function ToggleSettingsCard({
  title,
  description,
  items,
  onToggle,
  orientation = "vertical",
  className
}: ToggleSettingsCardProps) {
  return (
    <SettingsCard title={title} description={description} className={className}>
      <ToggleGroup
        items={items}
        onToggle={onToggle}
        orientation={orientation}
      />
    </SettingsCard>
  )
}

// Settings card with single toggle
export function SingleToggleCard({
  title,
  description,
  checked,
  onCheckedChange,
  loading,
  disabled,
  toggleLabel,
  toggleDescription,
  className
}: SingleToggleCardProps) {
  return (
    <SettingsCard title={title} description={description} className={className}>
      <div className="flex items-center justify-between">
        <div className="space-y-1">
          {toggleLabel && (
            <span className="text-sm font-medium">{toggleLabel}</span>
          )}
          {toggleDescription && (
            <p className="text-xs text-muted-foreground">{toggleDescription}</p>
          )}
        </div>
        <CompactToggle
          checked={checked}
          onCheckedChange={onCheckedChange}
          loading={loading}
          disabled={disabled}
        />
      </div>
    </SettingsCard>
  )
}