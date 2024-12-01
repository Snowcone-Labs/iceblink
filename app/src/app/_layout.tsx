import { Stack } from "expo-router";
import { usePreventScreenCapture } from "expo-screen-capture";
import { Platform } from "react-native";
import "../assets/global.css";

export default function RootLayout() {
  if (Platform.OS === "android" || Platform.OS === "ios") {
    usePreventScreenCapture();
  }

  return (
    <Stack
      screenOptions={{
        headerShown: false,
        contentStyle: { backgroundColor: "#271e41" },
      }}
    />
  );
}
