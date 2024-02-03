import { InputText } from "primereact/inputtext";
import { Button } from "primereact/button";

import { useForm, Controller } from "react-hook-form";
import { classNames } from "primereact/utils";
import { Password } from "primereact/password";

export type CredentialsInput = {
    username: string;
    password: string;
};

type Props = {
    onSubmit: (data: CredentialsInput) => void;
    error?: string;
    isPending?: boolean;
};

export const CredentialsLogin = ({ onSubmit, error, isPending }: Props) => {
    const {
        control,
        formState: { errors },
        handleSubmit,
    } = useForm<CredentialsInput>();

    return (
        <form onSubmit={handleSubmit(onSubmit)}>
            <h1 className="mx-auto w-fit">Login with your credentials</h1>

            <div className="flex flex-col w-fit mx-auto mt-4">
                <Controller
                    name="username"
                    control={control}
                    rules={{
                        required: "Username is required.",
                    }}
                    render={({ field, fieldState }) => (
                        <div className="flex flex-col gap-1">
                            <InputText
                                placeholder="Username"
                                id={field.name}
                                value={field.value}
                                className={classNames({ "p-invalid": fieldState.error })}
                                onChange={(e) => field.onChange(e.target.value)}
                            />
                            <small className="p-error">{errors.username?.message}</small>
                        </div>
                    )}
                />

                <Controller
                    name="password"
                    control={control}
                    rules={{ required: "Password is required." }}
                    render={({ field, fieldState }) => (
                        <div className="flex flex-col gap-2 mt-1">
                            <Password
                                placeholder="Password"
                                id={field.name}
                                value={field.value}
                                className={classNames({ "p-invalid": fieldState.error })}
                                onChange={(e) => field.onChange(e.target.value)}
                            />
                            <small className="p-error">{errors.password?.message}</small>
                        </div>
                    )}
                />

                <small className="p-error">{error}</small>

                <Button
                    loading={isPending}
                    className="mt-4"
                    type="submit"
                    label="Log in"
                />
            </div>
        </form>
    );
};
