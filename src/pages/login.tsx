import { useState } from "react"
import { CredentialsInput, CredentialsLogin } from "../component/CredentialsLogin"
import { invoke } from "@tauri-apps/api";

export const LoginPage = () => {
    const [value, setValue] = useState<string | null>(null);

    const onLogin = async (data: CredentialsInput) => {
        const response: string = await invoke("user_credentials_login", data);
        setValue(response);
    }

    return (
        <div>
            {value}
            <CredentialsLogin onSubmit={onLogin} />
        </div>
    )
}