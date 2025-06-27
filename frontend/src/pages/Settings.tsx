import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Switch } from "@/components/ui/switch";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { useState, useEffect } from "react";
import { useTheme } from '@/components/theme-provider';

export function Settings() {
  const [enableEmbeddings, setEnableEmbeddings] = useState(true);
  const [enableAutoCategorize, setEnableAutoCategorize] = useState(true);
  const { theme, setTheme } = useTheme();
  const [localTheme, setLocalTheme] = useState(theme);

  useEffect(() => setLocalTheme(theme), [theme]);

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Settings</h1>
        <p className="text-muted-foreground">Configure platform preferences</p>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>AI Analysis</CardTitle>
          <CardDescription>Enable or disable automated analyses</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center justify-between">
            <span className="text-sm">Generate Embeddings</span>
            <Switch checked={enableEmbeddings} onCheckedChange={setEnableEmbeddings} />
          </div>
          <div className="flex items-center justify-between">
            <span className="text-sm">Auto Categorization</span>
            <Switch checked={enableAutoCategorize} onCheckedChange={setEnableAutoCategorize} />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Appearance</CardTitle>
          <CardDescription>Personalize the UI</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <label className="text-sm font-medium">Theme</label>
            <Select value={localTheme} onValueChange={(val) => setLocalTheme(val as any)}>
              <SelectTrigger className="w-48">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="system">System</SelectItem>
                <SelectItem value="light">Light</SelectItem>
                <SelectItem value="dark">Dark</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      <div className="flex gap-2 justify-end">
        <Button variant="outline">Cancel</Button>
        <Button onClick={() => setTheme(localTheme as any)}>Save Changes</Button>
      </div>
    </div>
  );
} 