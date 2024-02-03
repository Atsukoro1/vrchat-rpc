import { TreeSelect, TreeSelectSelectionKeysType } from 'primereact/treeselect';
import { Header } from "../Header"
import { TreeNode } from 'primereact/treenode';
import { useState } from 'react';
import { Nullable } from 'primereact/ts-helpers';
import { EveryGame } from './components';
import { Divider } from 'primereact/divider';

const nodes: TreeNode[] = [
    {
        label: "Every game",
        key: "every_game",
        data: { component: <EveryGame /> }
    },
];

export const ConfigurationTab = () => {
    const [selectedNodeKey, setSelectedNodeKey] = useState<Nullable<string | TreeSelectSelectionKeysType | TreeSelectSelectionKeysType[]>>();

    return (
        <div className="p-4 flex flex-col gap-4 w-full overflow-scroll">
            <Header
                title="Configuration"
                description="Control how's your Discord status displayed."
            />

            <TreeSelect
                value={selectedNodeKey}
                onChange={(e) => {
                    console.log(e);
                    setSelectedNodeKey(e.value)
                }}
                options={nodes}
                className="md:w-20rem w-full"
                placeholder="Select Item"
            />
            <Divider className='mt-2 mb-2' />

            {nodes.find(node => node.key === selectedNodeKey)?.data.component}
        </div>
    )
}