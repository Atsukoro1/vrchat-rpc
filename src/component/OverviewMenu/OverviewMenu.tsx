import { Menu } from "primereact/menu";

export const OverviewMenu = () => {
    return (
        <div>
            <Menu
                className="h-screen"
                model={[
                    {
                        label: "Presences",
                        icon: "pi pi-fw pi-file",
                        items: [
                            { label: "Configuration", icon: "pi pi-fw pi-sliders-v", url: "/overview/configuration" },
                            { label: "Browse", disabled: true, icon: "pi pi-fw pi-folder", url: "/overview/browse" },
                        ],
                    },
                    {
                        label: "Other",
                        icon: "pi pi-fw pi-pencil",
                        items: [
                            { label: "Settings", icon: "pi pi-fw pi-cog", url: "/overview/settings" },
                            { label: "Profile", disabled: true, icon: "pi pi-fw pi-user", url: "/overview/profile" },
                        ],
                    },
                ]}
            />
        </div>
    );
};
