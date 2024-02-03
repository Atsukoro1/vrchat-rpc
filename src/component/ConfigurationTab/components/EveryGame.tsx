import { InputText } from "primereact/inputtext";
import { classNames } from "primereact/utils";
import { Controller, useForm } from "react-hook-form";
import { Header } from "../../Header";
import { Button } from "primereact/button";
import { Checkbox } from "primereact/checkbox";

type GameInfoInput = {
    title: string;
    description: string;
    showTimestamp: boolean;
    showPlayerStatus: boolean;
}

export const EveryGame = () => {
    const {
        control,
        formState: { errors },
        handleSubmit,
    } = useForm<GameInfoInput>();

    const onSubmit = (data: GameInfoInput) => {
        console.log(data);
    }

    return (
        <div className="flex flex-col gap-3">
            <Header
                title="For every game"
                description="This will be displayed for every game that you don't enable manually"
            />

            <form className="gap-3 flex flex-col" onSubmit={handleSubmit(onSubmit)}>
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
                    name="showPlayerStatus"
                    control={control}
                    rules={{
                        required: "Description is required.",
                    }}
                    render={({ field, fieldState }) => (
                        <div className="flex align-items-center">
                            <Checkbox
                                id={field.name}
                                onChange={(e) => field.onChange(e.checked)}
                                checked={field.value}
                                className={classNames({ "p-invalid": fieldState.error })}
                                inputId={field.name}
                            />
                            <label htmlFor={field.name} className="ml-2">Show player status</label>
                        </div>
                    )}
                />

                <Controller
                    name="showTimestamp"
                    control={control}
                    rules={{
                        required: "Description is required.",
                    }}
                    render={({ field, fieldState }) => (
                        <div className="flex align-items-center">
                            <Checkbox
                                id={field.name}
                                onChange={(e) => field.onChange(e.checked)}
                                checked={field.value}
                                className={classNames({ "p-invalid": fieldState.error })}
                                inputId={field.name}
                            />
                            <label htmlFor={field.name} className="ml-2">Show playtime</label>
                        </div>
                    )}
                />

                <Button
                    type="submit"
                    icon="pi pi-check"
                    label="Save"
                />
            </form>
        </div>
    )
}