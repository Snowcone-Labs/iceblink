import { Button } from "@/components/ui/Button";
import { Link } from "expo-router";
import { Text, View } from "react-native";

export default function UnlockPage() {
  return (
    <View className="bg-iceblink-bg-dark p-5 justify-center items-center flex-1 gap-6">
      <Text className="color-iceblink-fg-dark text-5xl">Customize server</Text>

      <Link href="/(auth)/signin" asChild>
        <Button color="primary">Done</Button>
      </Link>
    </View>
  );
}
