import { path } from "@tauri-apps/api";
import { BaseDirectory, exists, mkdir, readDir, readTextFile } from "@tauri-apps/plugin-fs";
import { defineStore } from "pinia";
import { ref } from "vue";

export interface PluginManifest {
  type: 'widget' | 'extension' | 'theme';
  isBuiltIn?: boolean;
  id: string;
  name: string;
  description?: string;
  version: string;
  homepage?: string;
  icon?: string;
  author?: {
    name?: string;
    email?: string;
  };
  widgetMeta?: {
    index: string;
    width: number;
    height: number;
  };
  isActivated?: boolean;
}

export const usePluginManager = defineStore('pluginManager', () => {
  const plugins = ref<PluginManifest[]>([])

  const _getPluginsFrom = async (baseDir: BaseDirectory, searchPath: string) => {
    let result: PluginManifest[] = []

    if (!await exists(searchPath, { baseDir })) {
      await mkdir(searchPath, { baseDir, recursive: true })
    }

    const entries = await readDir(searchPath, {
      baseDir,
    })

    for (const entry of entries) {
      if (entry.isDirectory) {
        const manifestPath = await path.join(
          searchPath,
          entry.name,
          'hbcat-manifest.json',
        );

        if (!(await exists(manifestPath, { baseDir }))) continue;

        const manifest = JSON.parse(
          await readTextFile(manifestPath, { baseDir }),
        );

        if (manifest.isBuiltIn) {
          delete manifest.isBuiltIn;
        }

        result.push(manifest);
      }
    }
    return result
  }

  const refreshPlugins = async () => {
    plugins.value = []

    const builtinPlugins = (await _getPluginsFrom(BaseDirectory.Resource, 'plugins')).map(p => {
      return {
        ...p,
        isBuiltIn: true,
      }
    })
    const userPlugins = await _getPluginsFrom(BaseDirectory.AppData, 'plugins')

    for (const plugin of [...builtinPlugins, ...userPlugins]) {
      const index = plugins.value.findIndex(p => p.id === plugin.id)
      if (index === -1) {
        plugins.value.push(plugin)
      } else {
        plugins.value[index] = {
          ...plugins.value[index],
          ...plugin,
        }
      }
    }
  }

  return {
    plugins,
    refreshPlugins,
  }
})
