import { Route, Routes } from "react-router-dom"

import { ConfigurationTab } from "../ConfigurationTab"
import { BrowseTab } from "../BrowseTab"
import { SettingsTab } from "../SettingsTab"
import { ProfileTab } from "../ProfileTab"

export const OverviewTabs = () => {
    return (
        <Routes>
            <Route path="configuration" element={<ConfigurationTab />} />
            <Route path="browse" element={<BrowseTab />} />
            <Route path="settings" element={<SettingsTab />} />
            <Route path="profile" element={<ProfileTab />} />
        </Routes>
    )
}