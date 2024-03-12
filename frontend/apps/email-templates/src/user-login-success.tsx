import { Body, Head, Html, Preview, Text } from "@react-email/components";
import { FONT_FAMILY } from "./shared/font";

export default function UserLoginSuccessEmail(): JSX.Element {
  return (
    <Html>
      <Head />
      <Preview>🔑 새로운 로그인이 확인되었습니다.</Preview>
      <Body>
        <Text style={text}>새로운 로그인이 확인되었습니다.</Text>
      </Body>
    </Html>
  );
}

const text = {
  fontFamily: FONT_FAMILY,
};
