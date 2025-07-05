import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { apiClient } from '@/lib/api'
import { AppSettings } from '@/types'

const SETTINGS_QUERY_KEY = ['settings']

export function useSettings() {
  const queryClient = useQueryClient()

  // Query for fetching current settings
  const settingsQuery = useQuery({
    queryKey: SETTINGS_QUERY_KEY,
    queryFn: () => apiClient.getSettings(),
    staleTime: 1000 * 60 * 5, // 5 minutes
    retry: 2,
  })

  // Mutation for updating settings
  const updateSettingsMutation = useMutation({
    mutationFn: (settings: Partial<AppSettings>) => 
      apiClient.updateSettings(settings),
    onSuccess: (updatedSettings) => {
      // Update the cache with new settings
      queryClient.setQueryData(SETTINGS_QUERY_KEY, updatedSettings)
      
      console.log('Settings saved successfully!')
    },
    onError: (error) => {
      console.error('Failed to update settings:', error)
    },
  })

  // Helper function to update a single setting
  const updateSetting = <K extends keyof AppSettings>(
    key: K, 
    value: AppSettings[K]
  ) => {
    updateSettingsMutation.mutate({ [key]: value })
  }

  // Helper function to update multiple settings
  const updateSettings = (settings: Partial<AppSettings>) => {
    updateSettingsMutation.mutate(settings)
  }

  return {
    // Data
    settings: settingsQuery.data,
    
    // Loading states
    isLoading: settingsQuery.isLoading,
    isUpdating: updateSettingsMutation.isPending,
    
    // Error states
    error: settingsQuery.error || updateSettingsMutation.error,
    
    // Actions
    updateSetting,
    updateSettings,
    
    // Utilities
    refetch: settingsQuery.refetch,
    reset: () => {
      queryClient.invalidateQueries({ queryKey: SETTINGS_QUERY_KEY })
    },
  }
}