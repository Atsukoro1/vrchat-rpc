import { invoke } from "@tauri-apps/api";
import { ProgressSpinner } from "primereact/progressspinner";
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";

type CheckUserResponse = {
	ok: boolean;
	token: string;
};

export const WelcomePage = () => {
	const navigate = useNavigate();

	const checkUser = async () => {
		const cookie = localStorage.getItem("authCookie");
		if (!cookie) return navigate("/login");

		try {
			const response: CheckUserResponse = await invoke("check_user", { auth: cookie });

			if (response.ok) {
				localStorage.setItem("authCookie", response.token);
				return navigate("/overview");
			}

			navigate("/login");
		} catch (error) {
			navigate("/login");
		}
	}

	useEffect(() => {
		checkUser();
	}, [checkUser]);

	return (
		<div className="text-center w-fit mx-auto">
			<h1 className="text-[#35c4dc] font-semibold">Loading</h1>
			<ProgressSpinner
				pt={{
					circle: {
						style: { stroke: "#35c4dc", strokeWidth: 8, animation: "none" },
					},
				}}
				style={{ width: "50px", height: "50px", color: "red" }}
				className="w-[50px] h-[50px] mt-2"
			/>
		</div>
	);
};
