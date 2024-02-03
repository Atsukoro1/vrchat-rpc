import { useState } from "react";
import {
	CredentialsInput,
	CredentialsLogin,
} from "../component/CredentialsLogin";
import { useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api";

type TwoFactorAuthType = "otp" | "totp" | "emailOtp";

export type CredentialsSuccessResponse = {
	authCookie: string;
	requiresTwoFactorAuth: TwoFactorAuthType[];
};

export type CredentialsErrorResponse = {
	message: string;
	status_code: number;
};

export const LoginPage = () => {
	const navigate = useNavigate();

	const [error, setError] = useState<string | undefined>();
	const [isLoading, setIsLoading] = useState(false);

	const saveAuthCookie = (authCookie: string) => {
		localStorage.setItem("authCookie", authCookie);
	};

	const conditionalRedirect = (
		requiresTwoFactorAuth: ("otp" | "totp" | "emailOtp")[],
	) => {
		if (requiresTwoFactorAuth.length === 0) {
			navigate("/overview");
			return;
		}

		if (requiresTwoFactorAuth.includes("emailOtp")) {
			navigate("/otp");
		}
	};

	const onLogin = async (data: CredentialsInput) => {
		try {
			setIsLoading(true);
			const response: CredentialsSuccessResponse = await invoke(
				"user_credentials_login",
				data,
			);
			setIsLoading(false);

			saveAuthCookie(response.authCookie);
			conditionalRedirect(response.requiresTwoFactorAuth);
		} catch (error) {
			setIsLoading(false);

			const response = error as CredentialsErrorResponse;
			setError(response.message);
		}
	};

	return (
		<CredentialsLogin error={error} onSubmit={onLogin} isPending={isLoading} />
	);
};
