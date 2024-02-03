import { Menu } from "primereact/menu"

export const OverviewMenu = () => {
    return (
        <div>
            <Menu
                className="h-screen"
                model={[
                    { label: 'File', icon: 'pi pi-fw pi-file', items: [{ label: 'New', icon: 'pi pi-fw pi-plus' }, { label: 'Open', icon: 'pi pi-fw pi-download' }] },
                    { label: 'Edit', icon: 'pi pi-fw pi-pencil', items: [{ label: 'Undo', icon: 'pi pi-fw pi-refresh' }, { label: 'Redo', icon: 'pi pi-fw pi-replay' }] }
                ]}
            />
        </div>
    )
}