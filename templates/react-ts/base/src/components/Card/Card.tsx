import ExternalLink from "../../assets/ExternalLink.svg";
import "./Card.css";

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
      className="card"
    >
      <div className="card-header">
        <h3 className="card-title">{title}</h3>
        {href && (
          <img
            src={ExternalLink}
            alt="External Link Icon"
            className="card-icon"
          />
        )}
      </div>
      <p className="card-description">{description}</p>
    </a>
  );
};

export default Card;
