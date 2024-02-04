import { Controller, useForm } from "react-hook-form";
import { Checkbox } from "primereact/checkbox";
import { Header } from "../Header";
import { Button } from "primereact/button";
import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api";
import { Toast } from "primereact/toast";

type SettingsInput = {
    minimizeToTray: boolean;
    startOnStartup: boolean;
}

export const SettingsTab = () => {
    const toast = useRef<Toast>(null);

    const {
        control,
        handleSubmit,
        setValue,
    } = useForm<SettingsInput>();

    const onLoadGetSettings = async () => {
        try {
            const settings: SettingsInput = await invoke("get_settings");

            setValue("minimizeToTray", settings.minimizeToTray);
            setValue("startOnStartup", settings.startOnStartup);
        } catch (error) {
            toast.current?.show({
                severity: 'error',
                summary: 'Error',
                detail: 'Something went wrong while fetching settings. Please try again later.',
            });
        }
    }

    useEffect(() => {
        onLoadGetSettings();
    }, [onLoadGetSettings]);

    const [isSaving, setIsSaving] = useState<boolean>(false);
    const onSaveSettings = async (data: SettingsInput) => {
        setIsSaving(true);

        try {
            await invoke("set_settings", { data });
            toast.current?.show({
                severity: 'success',
                summary: 'Settings saved',
            });
        } catch (err) {
            toast.current?.show({
                severity: 'error',
                summary: 'Failed to save settings',
                detail: 'Something went wrong, please try again later.',
            });
        } finally {
            setIsSaving(false);
        }
    }

    return (
        <div className="p-4 flex flex-col gap-4">
            <Header
                title="Version"
                description="1.0.0"
            />
            <Header
                title="Repository URL"
                description="https://github.com/Atsukoro1/vrchat-rpc"
            />
            <Header
                title="Support"
                description="Github or contact atsukoro on discord"
            />

            <form className="gap-3 flex flex-col mt-4" onSubmit={handleSubmit(onSaveSettings)}>
                <Controller
                    name="minimizeToTray"
                    control={control}
                    render={({ field }) => (
                        <div className="flex flex-row gap-1 items-center">
                            <Checkbox
                                id={field.name}
                                onChange={(e) => field.onChange(e.checked)}
                                checked={field.value}
                                inputId={field.name}
                            />
                            <label htmlFor={field.name} className="ml-2">Minimize to tray</label>
                        </div>
                    )}
                />

                <Controller
                    name="startOnStartup"
                    control={control}
                    render={({ field }) => (
                        <div className="flex flex-row gap-1 items-center">
                            <Checkbox
                                id={field.name}
                                onChange={(e) => field.onChange(e.checked)}
                                checked={field.value}
                                inputId={field.name}
                            />
                            <label htmlFor={field.name} className="ml-2">
                                Start on Windows startup
                            </label>
                        </div>
                    )}
                />

                <Button
                    loading={isSaving}
                    type="submit"
                    label="Save settings"
                />
            </form>

            <Toast ref={toast} />
        </div >
    )
}