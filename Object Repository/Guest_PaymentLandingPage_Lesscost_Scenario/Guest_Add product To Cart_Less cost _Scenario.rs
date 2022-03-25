<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Guest_Add product To Cart_Less cost _Scenario</name>
   <tag></tag>
   <elementGuidId>4959c8ba-6165-4bee-b0d5-a184adc52758</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;qty&quot;,
      &quot;value&quot;: &quot;1&quot;
    },
    {
      &quot;name&quot;: &quot;code&quot;,
      &quot;value&quot;: &quot;${productCode_lesscost}&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
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
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${BaseURL}/services/v2_1/ssl/users/anonymous/carts/${GUID_Less}/entries</restUrl>
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
      <id>d24bbf66-5e26-4671-8a95-d186820546e4</id>
      <masked>false</masked>
      <name>BaseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.GUID_Less</defaultValue>
      <description></description>
      <id>ffb97eef-62f1-4742-840c-59fb3742959f</id>
      <masked>false</masked>
      <name>GUID_Less</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guest_Access_Token</defaultValue>
      <description></description>
      <id>a51cb717-c5c2-47f0-96f3-ec0bc18e6fee</id>
      <masked>false</masked>
      <name>guest_Access_Token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.productCode_lesscost</defaultValue>
      <description></description>
      <id>1fa53296-e424-4bd4-853a-ec397d327686</id>
      <masked>false</masked>
      <name>productCode_lesscost</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
