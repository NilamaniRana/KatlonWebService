<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Guest_Get_GUID_Details_After Checkout _Less cost _Scenario</name>
   <tag></tag>
   <elementGuidId>4809a8bf-4417-46d6-9d86-b453353c59c4</elementGuidId>
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
      <id>0367da58-d11c-4dfa-affd-ffa663520c2d</id>
      <masked>false</masked>
      <name>BaseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.GUID_Less</defaultValue>
      <description></description>
      <id>cc671995-db81-43f0-a225-88f284defa85</id>
      <masked>false</masked>
      <name>GUID_Less</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guest_Access_Token</defaultValue>
      <description></description>
      <id>cf5c0a06-99e8-468b-b128-05fa09d45f15</id>
      <masked>false</masked>
      <name>guest_Access_Token</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>