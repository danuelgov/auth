import { Body, Head, Html, Link, Preview, Text } from "@react-email/components";
import { FONT_FAMILY } from "./shared/font";

export default function SignupRequestEmail(): JSX.Element {
  return (
    <Html>
      <Head />
      <Preview>🔑 회원가입 인증을 완료해 주세요.</Preview>
      <Body>
        <Text style={text}>
          아래 링크를 선택하면 다뉴엘 거버넌스에 {"{{EMAIL_ADDRESS}}"} 계정
          생성이 완료 됩니다.
        </Text>
        <Link style={text} href={"{{VERFICATION_URL}}"}>
          이메일 인증 완료하기
        </Link>
        <Text style={text}>(이 링크는 15분 이후 만료 됩니다.)</Text>
        <Text style={text}>
          링크가 작동하지 않는 경우, 아래 링크를 직접 브라우저 주소창에 입력해
          주세요.
        </Text>
        <Link style={text} href={"{{VERFICATION_URL}}"}>
          {"{{VERFICATION_URL}}"}
        </Link>
        <Text style={text}>
          이 이메일은 다뉴엘 거버넌스에서 {"{{EMAIL_ADDRESS}}"} 계정 생성을
          요청한 것으로 확인됩니다.
        </Text>
        <Text style={text}>
          이 이메일을 받은 적이 없다면, 이 이메일을 무시해 주세요.
        </Text>
      </Body>
    </Html>
  );
}

const text = {
  fontFamily: FONT_FAMILY,
};
