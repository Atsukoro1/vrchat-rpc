type Props = {
    title: string | JSX.Element;
    description?: string | JSX.Element;
};

export const Header = ({ title, description }: Props) => {
    return (
        <div className="flex flex-col">
            <h1 className="text-xl font-bold text-gray-700">{title}</h1>
            <p className="text-gray-600">{description}</p>
        </div>
    )
}