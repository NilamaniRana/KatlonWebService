import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

response = WS.sendRequest(findTestObject('Guest_PaymentLandingPage_Lesscost_Scenario/Guest_AcessToken', [('BaseURL') : GlobalVariable.BaseURL
            , ('ClientSecret') : GlobalVariable.client_secret]))

def accessToken = CustomKeywords.'newpakage.newkeyword.getAccessToken'('Guest_PaymentLandingPage_Lesscost_Scenario/Guest_AcessToken')

//def accessToken = WS.getElementPropertyValue(response, 'access_token')

println(accessToken)

guidresponse = WS.sendRequest(findTestObject('Guest_PaymentLandingPage_Lesscost_Scenario/Guest_Get_GUID_Details_Set GUID_Less cost _Scenario', [('BaseURL') :
  GlobalVariable.BaseURL, ('guest_Access_Token') : accessToken]))
  
def guid = WS.getElementPropertyValue(guidresponse, 'guid')
  
println(guid)
  
WS.sendRequest(findTestObject('Guest_PaymentLandingPage_Lesscost_Scenario/Guest_Get_GUID_Details copy_Less cost _Scenario', [('BaseURL') :GlobalVariable.BaseURL, ('GUID_Less') : guid, ('guest_Access_Token') :accessToken]))
  
WS.sendRequest(findTestObject('Guest_PaymentLandingPage_Lesscost_Scenario/Guest_Add product To Cart_Less cost _Scenario', [('BaseURL') :
  GlobalVariable.BaseURL, ('GUID_Less') : guid, ('guest_Access_Token') :
  accessToken , ('productCode_lesscost') :
  GlobalVariable.productCode_lesscost]))
  
WS.sendRequest(findTestObject('Guest_PaymentLandingPage_Lesscost_Scenario/Guest_Get_GUID_Details_After adding product to Cart _Less cost _Scenario',
  [('BaseURL') : GlobalVariable.BaseURL, ('GUID_Less') : guid,
  ('guest_Access_Token') : accessToken]))
  
  WS.sendRequest(findTestObject('Guest_PaymentLandingPage_Lesscost_Scenario/Test_Successful_Guest_User_Checkout_Less cost _Scenario', [('BaseURL') :
  GlobalVariable.BaseURL, ('GUID_Less') : guid, ('guest_Access_Token') :
  accessToken]))
  
  WS.sendRequest(findTestObject('Guest_PaymentLandingPage_Lesscost_Scenario/Test_Successfully_Add_Address_Less cost _Scenario', [('BaseURL') :
  GlobalVariable.BaseURL, ('GUID_Less') : guid, ('guest_Access_Token') :
  accessToken]))
  
  WS.sendRequest(findTestObject('Guest_PaymentLandingPage_Lesscost_Scenario/Guest_Get_GUID_Details_After Checkout _Less cost _Scenario', [('BaseURL') :
  GlobalVariable.BaseURL, ('GUID_Less') : guid, ('guest_Access_Token') :
  accessToken]))
  
  WS.sendRequest(findTestObject('Guest_PaymentLandingPage_Lesscost_Scenario/Guest_Payment page_Less cost _Scenario', [('BaseURL') :
  GlobalVariable.BaseURL , ('GUID_Less') : guid, ('guest_Access_Token') :
  accessToken]))
  
  WS.sendRequest(findTestObject('Guest_PaymentLandingPage_Lesscost_Scenario/Guest_Get_Hash_API', [('BaseURL') : GlobalVariable.BaseURL , ('GUID_Less') :
  guid, ('guest_Access_Token') : accessToken]))
  
  WS.sendRequest(findTestObject('Guest_PaymentLandingPage_Lesscost_Scenario/Guest_Place cod order_Invalid accessToken_low cost product', [('BaseURL') :
  GlobalVariable.BaseURL, ('GUID_Less') : guid]))
  
  WS.sendRequest(findTestObject('Guest_PaymentLandingPage_Lesscost_Scenario/Guest_Place cod order_Invalid Guid_low cost product', [('BaseURL') :
  GlobalVariable.BaseURL, ('guest_Access_Token') : accessToken]))
  
 