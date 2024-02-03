import { Controller, useForm } from "react-hook-form";
import { Checkbox } from "primereact/checkbox";
import { Header } from "../Header";
import { Button } from "primereact/button";

type SettingsInput = {
    minimizeToTray: boolean;
    startOnStartup: boolean;
}

export const SettingsTab = () => {
    const {
        control,
        handleSubmit,
    } = useForm<SettingsInput>();

    const onSubmit = (data: SettingsInput) => {
        console.log(data);
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

            <form className="gap-3 flex flex-col mt-4" onSubmit={handleSubmit(onSubmit)}>
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
                    type="submit"
                    label="Save settings"
                />
            </form>

        </div >
    )
}