export type AppState = {
    isLoading: false;
    settings: Settings;
} | {
    isLoading: true;
};

export interface LoadResult {
    settings: Settings
}

export interface Settings {
    
}