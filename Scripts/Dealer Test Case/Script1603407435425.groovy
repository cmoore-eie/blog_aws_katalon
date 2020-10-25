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
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

WS.sendRequest(findTestObject('Local/Dealer/Dealer Create', [('uuid') : 'UUID31', ('code') : 'd1', ('name') : 'Dealer Name 1']), 
    FailureHandling.STOP_ON_FAILURE)

WS.sendRequest(findTestObject('Local/Dealer/Dealer Read', [('uuid') : 'UUID31']), FailureHandling.STOP_ON_FAILURE)

WS.sendRequest(findTestObject('Local/Dealer/Dealer Update', [('uuid') : 'UUID31', ('code') : 'd1', ('name') : 'Dealer Name 1 Changed']), 
    FailureHandling.STOP_ON_FAILURE)

WS.sendRequest(findTestObject('Local/Dealer/Dealer List All', [('uuid') : 'UUID21']), FailureHandling.STOP_ON_FAILURE)

WS.sendRequest(findTestObject('Local/Dealer/Dealer Delete', [('uuid') : 'UUID31']), FailureHandling.STOP_ON_FAILURE)

