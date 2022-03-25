<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Guest_Get_GUID_Details_After adding product to Cart _Less cost _Scenario</name>
   <tag></tag>
   <elementGuidId>1715e9e8-3446-4a47-825e-1829ffd20c28</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${guest_Access_Token}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${BaseURL}/services/v2_1/ssl/users/anonymous/carts/${GUID_Less}</restUrl>
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
      <id>3622c268-5f2f-487e-85fd-298d7f6e4f3c</id>
      <masked>false</masked>
      <name>BaseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.GUID_Less</defaultValue>
      <description></description>
      <id>ae2453a8-c17f-4ffe-b854-f22ccb527450</id>
      <masked>false</masked>
      <name>GUID_Less</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guest_Access_Token</defaultValue>
      <description></description>
      <id>2d9beedd-1525-4c68-8b78-b5872ce88441</id>
      <masked>false</masked>
      <name>guest_Access_Token</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>