import ExternalLink from "../assets/ExternalLink.svg";

interface CardProps {
  title: string;
  description: string;
  href?: string;
}

const Card: React.FC<CardProps> = ({ title, description, href }) => {
  return (
    <a
      href={href || "#"}
      target={href ? "_blank" : "_self"}
      rel="noreferrer"
      className="bg-[#1e293b] rounded-lg p-6 border border-slate-700 hover:border-blue-500 transition-colors duration-200 ease-in-out group shadow-sm"
    >
      <div className="flex items-start justify-between mb-2">
        <h3 className="text-lg font-semibold text-slate-200">{title}</h3>
        {href && (
          <img
            src={ExternalLink}
            alt="External Link Icon"
            className="w-4 h-4 text-slate-500 group-hover:text-blue-400 transition-colors"
          />
        )}
      </div>
      <p className="text-slate-400 text-sm">{description}</p>
    </a>
  );
};

export default Card;
