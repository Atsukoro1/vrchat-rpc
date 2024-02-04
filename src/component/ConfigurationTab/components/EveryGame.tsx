import { InputText } from "primereact/inputtext";
import { classNames } from "primereact/utils";
import { Controller, useForm } from "react-hook-form";
import { Header } from "../../Header";
import { Button } from "primereact/button";
import { Checkbox } from "primereact/checkbox";
import { invoke } from "@tauri-apps/api";
import { useEffect, useRef, useState } from "react";
import { Toast } from 'primereact/toast';

type Configuration = {
    title: string;
    description: string;
    showTimestamp: boolean;
    showPlayerStatus: boolean;
    largeImageKey: string;
    smallImageKey: string;
}

export const EveryGame = () => {
    const {
        control,
        setValue,
        formState: { errors },
        handleSubmit,
    } = useForm<Configuration>();

    const toastRef = useRef<Toast>(null);
    const onShowUploadedToast = () => {
        toastRef.current?.show({
            severity: 'success',
            summary: 'Success',
            detail: 'Configuration saved'
        });
    };

    const fetchInitialData = async () => {
        try {
            const initialData: Configuration = await invoke("get_configuration");

            setValue("title", initialData.title);
            setValue("description", initialData.description);
            setValue("showPlayerStatus", initialData.showPlayerStatus);
            setValue("showTimestamp", initialData.showTimestamp);
            setValue("largeImageKey", initialData.largeImageKey);
            setValue("smallImageKey", initialData.smallImageKey);
        } catch (error) {
            console.error("Error fetching initial data:", error);
        }
    };

    const [saving, setSaving] = useState<boolean>(false);
    const onSave = async (data: Configuration) => {
        setSaving(true);

        try {
            await invoke("set_configuration", { data });
            onShowUploadedToast();
        } catch (error) {
            console.error("Error saving configuration:", error);
        } finally {
            setSaving(false);
        }
    };

    useEffect(() => {
        fetchInitialData();
    }, [fetchInitialData]);

    return (
        <div className="flex flex-col gap-3">
            <Header
                title="For every game"
                description="This will be displayed for every game that you don't enable manually"
            />

            <form className="gap-3 flex flex-col" onSubmit={handleSubmit(onSave)}>
                <Controller
                    name="title"
                    control={control}
                    rules={{
                        required: "Title is required.",
                    }}
                    render={({ field, fieldState }) => (
                        <div className="flex flex-col gap-1">
                            <InputText
                                placeholder="Title"
                                id={field.name}
                                value={field.value}
                                className={classNames({ "p-invalid": fieldState.error })}
                                onChange={(e) => field.onChange(e.target.value)}
                            />
                            <small className="p-error">{errors.title?.message}</small>
                        </div>
                    )}
                />

                <Controller
                    name="description"
                    control={control}
                    rules={{
                        required: "Description is required.",
                    }}
                    render={({ field, fieldState }) => (
                        <div className="flex flex-col gap-1">
                            <InputText
                                placeholder="Description"
                                id={field.name}
                                value={field.value}
                                className={classNames({ "p-invalid": fieldState.error })}
                                onChange={(e) => field.onChange(e.target.value)}
                            />
                            <small className="p-error">{errors.description?.message}</small>
                        </div>
                    )}
                />

                <Controller
                    name="smallImageKey"
                    control={control}
                    render={({ field, fieldState }) => (
                        <div className="flex flex-col gap-1">
                            <InputText
                                placeholder="Small image text"
                                id={field.name}
                                value={field.value}
                                className={classNames({ "p-invalid": fieldState.error })}
                                onChange={(e) => field.onChange(e.target.value)}
                            />
                            <small className="p-error">{errors.smallImageKey?.message}</small>
                        </div>
                    )}
                />

                <Controller
                    name="largeImageKey"
                    control={control}
                    render={({ field, fieldState }) => (
                        <div className="flex flex-col gap-1">
                            <InputText
                                placeholder="Large image text"
                                id={field.name}
                                value={field.value}
                                className={classNames({ "p-invalid": fieldState.error })}
                                onChange={(e) => field.onChange(e.target.value)}
                            />
                            <small className="p-error">{errors.largeImageKey?.message}</small>
                        </div>
                    )}
                />

                <Controller
                    name="showPlayerStatus"
                    control={control}
                    render={({ field }) => (
                        <div className="flex align-items-center">
                            <Checkbox
                                id={field.name}
                                onChange={(e) => field.onChange(e.checked)}
                                checked={field.value}
                                inputId={field.name}
                            />
                            <label htmlFor={field.name} className="ml-2">Show player status</label>
                        </div>
                    )}
                />

                <Controller
                    name="showTimestamp"
                    control={control}
                    render={({ field }) => (
                        <div className="flex align-items-center">
                            <Checkbox
                                id={field.name}
                                onChange={(e) => field.onChange(e.checked)}
                                checked={field.value}
                                inputId={field.name}
                            />
                            <label htmlFor={field.name} className="ml-2">Show playtime</label>
                        </div>
                    )}
                />

                <Button
                    loading={saving}
                    type="submit"
                    icon="pi pi-check"
                    label="Save"
                />
            </form>

            <Toast ref={toastRef} />
        </div>
    )
}