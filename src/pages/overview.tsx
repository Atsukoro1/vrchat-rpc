import { OverviewMenu } from "../component/OverviewMenu";
import { OverviewTabs } from "../component/OverviewTabs";
import { OverviewWarning } from "../component/OverviewWarning/OverviewWarning";

export const OverviewPage = () => {
	return (
		<>
			<OverviewWarning />

			<div className="flex flex-row">
				<OverviewMenu />
				<OverviewTabs />
			</div>
		</>
	);
};
