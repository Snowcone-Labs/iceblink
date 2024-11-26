import { padding } from "@/utils/padding";
import { Image, Text, View } from "react-native";

export function TotpCard({
  displayName,
  logo,
  secret,
}: {
  displayName: string;
  logo: string;
  secret: string;
}) {
  return (
    <View
      style={{
        ...padding(18, 18, 22, 18),
        borderColor: "#1d1631",
        borderWidth: 1,
      }}
      className="flex items-center gap-5 self-stretch rounded-lg"
    >
      <View>
        <Text>{displayName}</Text>
        <Text>197 455</Text>
      </View>
      <View>
        <Image src={logo} alt={`Logo of ${displayName}`} />
      </View>
    </View>
  );
}
