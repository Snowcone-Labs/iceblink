import { Lightbulb } from "lucide-react-native";

export default function Tip({
  content,
  type,
}: {
  content: string;
  type: "warning" | "success" | "info" | "danger";
}) {
  // Todo: implement type field, idk how to make tailwind happy :)
  return (
    <p
      className={`text-xs text-iceblink-fg-warning bg-iceblink-bg-warning mt-1 p-4 rounded-md`}
    >
      <Lightbulb /> {content}
    </p>
  );
}
