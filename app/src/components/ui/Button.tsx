import React from "react";
import {
  StyleSheet,
  Text,
  TextStyle,
  TouchableOpacity,
  ViewStyle,
} from "react-native";

const colors = {
  danger: {
    bg: "#521111",
    fg: "#ff4f4f",
  },
  warning: {
    bg: "#6b450c",
    fg: "#ffbf42",
  },
  success: {
    bg: "#0c6b43",
    fg: "#20fea1",
  },
  primary: {
    bg: "hsla(253, 67%, 63%, 1)",
    fg: "white",
  },
  secondary: {
    bg: "#2f314b",
    fg: "white",
  },
};

interface Props {
  children?: React.ReactNode;
  icon?: React.ElementType;
  color?: keyof typeof colors;
  background?: string;
  foreground?: string;
  shadow?: boolean;
  disableRaiseOnFocus?: boolean;
  onPress?: () => void;
  style?: ViewStyle;
  textStyle?: TextStyle;
  disabled?: boolean;
  className?: string;
}

export function Button({
  children,
  color = "primary",
  background,
  foreground,
  shadow = false,
  disableRaiseOnFocus = false,
  onPress,
  style,
  textStyle,
  disabled = false,
  className,
  icon,
}: Props) {
  const buttonStyle: ViewStyle = {
    backgroundColor: background || colors[color].bg,
    display: "flex",
    flexDirection: "row",
    justifyContent: "space-between",
    alignItems: "center",
    borderWidth: 1,
    borderColor: "#fff2",
    paddingTop: 12,
    paddingBottom: 12,
    paddingLeft: children ? 20 : 12,
    paddingRight: children ? 20 : 12,
    borderRadius: 10,
    borderTopWidth: 1,
    borderTopColor: "rgba(255, 255, 255, 0.20)",
    boxShadow:
      " 0px 0px 0px 1px rgba(0, 0, 0, 0.25), 0px 5px 10px 0px rgba(0, 0, 0, 0.15)",
    ...(shadow && {
      shadowColor: "#000",
      shadowOffset: { width: 0, height: 2 },
      shadowOpacity: 0.2,
      shadowRadius: 2,
    }),
    ...style,
  };

  const textStyles: TextStyle = {
    color: foreground || colors[color].fg,
    fontWeight: 500,
    textAlign: "center",
    fontSize: 20,
    ...textStyle,
  };

  return (
    <TouchableOpacity
      onPress={onPress}
      style={[buttonStyle, disableRaiseOnFocus ? {} : styles.raiseOnFocus]}
      className={className}
      activeOpacity={0.8}
      disabled={disabled}
    >
      {children && <Text style={textStyles}>{children}</Text>}
      {icon &&
        React.createElement(icon, { style: { color: textStyles.color } })}
    </TouchableOpacity>
  );
}

const styles = StyleSheet.create({
  raiseOnFocus: {
    boxShadow:
      "0px 0px 0px 1px rgba(0, 0, 0, 0.25), var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)",
  },
});
