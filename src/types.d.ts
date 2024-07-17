declare module "vue3-snackbar" {
  import { App } from "vue";
  export class SnackbarService {
    static install(app: App): void;
  }

  export const Vue3Snackbar: any;

  export interface useSnackbarOptions {
    type?: "success" | "error" | "warning" | "info";
    background?: string;
    title?: string;
    text: string;
    dismissable?: boolean;
    icon?: Object;
    action?: Object;
  }

  export function useSnackbar(): {
    add: (options: useSnackbarOptions) => void;
    clear(): void;
  };
}
