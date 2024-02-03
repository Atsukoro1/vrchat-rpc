import { ProgressSpinner } from "primereact/progressspinner";
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";

export const WelcomePage = () => {
	const navigate = useNavigate();

	useEffect(() => {
		const timeout = setTimeout(() => navigate("/overview"), 1000);
		return () => clearTimeout(timeout);
	}, [navigate]);

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
