import { Button } from "primereact/button";
import { InputText } from "primereact/inputtext";
import { classNames } from "primereact/utils";
import { Controller, SubmitHandler, useForm } from "react-hook-form";

type CodeInput = {
    code: string;
}

export const EmailOtpVerify = () => {
    const {
        control,
        formState: { errors },
        handleSubmit,
    } = useForm<CodeInput>();

    const onSubmit: SubmitHandler<CodeInput> = (data) => console.log(data);

    return (
        <form onSubmit={handleSubmit(onSubmit)}>
            <h1 className="mx-auto w-fit">Verify the email code</h1>

            <div className="flex flex-col w-fit mx-auto mt-4">
                <Controller
                    name="code"
                    control={control}
                    rules={{
                        required: 'Code is required.',
                        maxLength: { value: 6, message: 'Maximal length of the code should be 6 digits.' },
                        minLength: { value: 6, message: 'Minimal length of the code should be 6 digits.' },
                        pattern: { value: /^[0-9]*$/, message: 'Code should contain only numbers.' }
                    }}
                    render={({ field, fieldState }) => (
                        <div className="flex flex-col gap-1">
                            <InputText placeholder="Email code" id={field.name} value={field.value} className={classNames({ 'p-invalid': fieldState.error })} onChange={(e) => field.onChange(e.target.value)} />
                            <small className="p-error">{errors.code?.message}</small>
                        </div>
                    )}
                />

                <Button className="mt-4" type="submit" label="Log in" />
            </div>
        </form>
    );
}