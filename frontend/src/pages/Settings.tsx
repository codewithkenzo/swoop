import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { LoadingSwitch } from "@/components/ui/loading-switch";
import { LoadingButton } from "@/components/ui/loading-button";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { useState, useEffect } from "react";
import { useTheme } from '@/components/theme-provider';
import { useSettings } from '@/hooks/useSettings';
import { AppSettings } from '@/types';
import { Loader2, AlertCircle } from "lucide-react";
import { Alert, AlertDescription } from "@/components/ui/alert";

export function Settings() {
  const { theme, setTheme } = useTheme();
  const { 
    settings, 
    isLoading, 
    isUpdating, 
    error, 
    updateSetting, 
    updateSettings 
  } = useSettings();

  // Local state for form values
  const [localSettings, setLocalSettings] = useState<Partial<AppSettings>>({});
  const [hasChanges, setHasChanges] = useState(false);

  // Initialize local settings when server settings load
  useEffect(() => {
    if (settings) {
      setLocalSettings(settings);
    }
  }, [settings]);

  // Track changes
  useEffect(() => {
    if (settings) {
      const changed = Object.keys(localSettings).some(
        key => localSettings[key as keyof AppSettings] !== settings[key as keyof AppSettings]
      );
      setHasChanges(changed);
    }
  }, [localSettings, settings]);

  // Handle individual setting changes with immediate save
  const handleToggleChange = (key: keyof AppSettings, value: boolean) => {
    updateSetting(key, value);
    setLocalSettings(prev => ({ ...prev, [key]: value }));
  };

  // Handle theme change (update both local theme and settings)
  const handleThemeChange = (newTheme: AppSettings['theme']) => {
    setLocalSettings(prev => ({ ...prev, theme: newTheme }));
    updateSetting('theme', newTheme);
    setTheme(newTheme);
  };

  // Save all changes
  const handleSaveChanges = () => {
    if (hasChanges) {
      const changedSettings = Object.keys(localSettings).reduce(
        (acc, key) => {
          const k = key as keyof AppSettings;
          if (settings && localSettings[k] !== settings[k]) {
            acc[k] = localSettings[k]!;
          }
          return acc;
        },
        {} as Partial<AppSettings>
      );
      
      updateSettings(changedSettings);
    }
  };

  // Reset changes
  const handleCancel = () => {
    if (settings) {
      setLocalSettings(settings);
      setHasChanges(false);
    }
  };

  // Loading state
  if (isLoading) {
    return (
      <div className="space-y-6">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Settings</h1>
          <p className="text-muted-foreground">Configure platform preferences</p>
        </div>
        <div className="flex items-center justify-center py-12">
          <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
          <span className="ml-2 text-muted-foreground">Loading settings...</span>
        </div>
      </div>
    );
  }

  // Error state
  if (error) {
    return (
      <div className="space-y-6">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Settings</h1>
          <p className="text-muted-foreground">Configure platform preferences</p>
        </div>
        <Alert variant="destructive">
          <AlertCircle className="h-4 w-4" />
          <AlertDescription>
            Failed to load settings. Please refresh the page or try again later.
          </AlertDescription>
        </Alert>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Settings</h1>
        <p className="text-muted-foreground">Configure platform preferences</p>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Crawling Options</CardTitle>
          <CardDescription>Configure advanced web crawling features</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center justify-between">
            <div className="space-y-1">
              <span className="text-sm font-medium">Advanced Crawl Features</span>
              <p className="text-xs text-muted-foreground">
                Enable JavaScript rendering and deep content extraction
              </p>
            </div>
            <LoadingSwitch 
              checked={localSettings.advanced_crawl ?? false}
              onCheckedChange={(checked) => handleToggleChange('advanced_crawl', checked)}
              loading={isUpdating}
            />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Notifications</CardTitle>
          <CardDescription>Manage notification preferences</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center justify-between">
            <div className="space-y-1">
              <span className="text-sm font-medium">Enable Notifications</span>
              <p className="text-xs text-muted-foreground">
                Receive notifications for completed operations and system updates
              </p>
            </div>
            <LoadingSwitch 
              checked={localSettings.notifications ?? true}
              onCheckedChange={(checked) => handleToggleChange('notifications', checked)}
              loading={isUpdating}
            />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Appearance</CardTitle>
          <CardDescription>Personalize the UI experience</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <label className="text-sm font-medium">Theme</label>
            <Select 
              value={localSettings.theme ?? 'system'} 
              onValueChange={handleThemeChange}
              disabled={isUpdating}
            >
              <SelectTrigger className="w-48">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="system">System</SelectItem>
                <SelectItem value="light">Light</SelectItem>
                <SelectItem value="dark">Dark</SelectItem>
              </SelectContent>
            </Select>
            <p className="text-xs text-muted-foreground">
              Choose your preferred color scheme
            </p>
          </div>
        </CardContent>
      </Card>

      {hasChanges && (
        <div className="flex gap-2 justify-end">
          <Button variant="outline" onClick={handleCancel} disabled={isUpdating}>
            Cancel
          </Button>
          <LoadingButton 
            onClick={handleSaveChanges}
            loading={isUpdating}
            loadingText="Saving..."
          >
            Save Changes
          </LoadingButton>
        </div>
      )}

      {!hasChanges && !isUpdating && (
        <div className="flex justify-end">
          <p className="text-sm text-muted-foreground py-2">
            Settings are automatically saved
          </p>
        </div>
      )}
    </div>
  );
} 