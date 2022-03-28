<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Test_Successfully_Add_Address_Less cost _Scenario</name>
   <tag></tag>
   <elementGuidId>c1a0fc53-1715-4d4b-877f-e5c3a397583d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n  \&quot;fullName\&quot;: \&quot;Rahul\&quot;,\r\n  \&quot;line1\&quot;: \&quot;Sector 223\&quot;,\r\n  \&quot;town\&quot;: \&quot;New Delhi\&quot;,\r\n  \&quot;postalCode\&quot;: \&quot;110045\&quot;,\r\n  \&quot;phone\&quot;: \&quot;8899776655\&quot;,\r\n  \&quot;defaultAddress\&quot;: true,\r\n  \&quot;region\&quot;: {\r\n    \&quot;isocode\&quot;: \&quot;IN-DL\&quot;\r\n  },\r\n  \&quot;country\&quot;: {\r\n    \&quot;isocode\&quot;:\&quot;IN\&quot;\r\n  }\r\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${guest_Access_Token}</value>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${BaseURL}/services/v2_1/ssl/users/anonymous/carts/${GUID_Less}/addresses/delivery</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.BaseURL</defaultValue>
      <description></description>
      <id>1fa4440d-b1f3-43d1-8a89-50ffcbcf7f5f</id>
      <masked>false</masked>
      <name>BaseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.GUID_Less</defaultValue>
      <description></description>
      <id>4439b06b-4596-4c9c-8c9e-0362cebcce92</id>
      <masked>false</masked>
      <name>GUID_Less</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guest_Access_Token</defaultValue>
      <description></description>
      <id>4c97d2f1-22e9-4ef6-aee0-7f850143014a</id>
      <masked>false</masked>
      <name>guest_Access_Token</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
